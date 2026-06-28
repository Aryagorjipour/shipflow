use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::error::{Result, ShipflowError};
use crate::git;
use crate::task::{SCHEMA_VERSION, Task, TaskFile, TaskStatus, resolve_task};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageMode {
    Repo(PathBuf),
    Global(PathBuf),
}

impl StorageMode {
    pub fn tasks_path(&self) -> &Path {
        match self {
            Self::Repo(p) | Self::Global(p) => p,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Repo(_) => "repo",
            Self::Global(_) => "global",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageContext {
    pub mode: StorageMode,
    pub tasks_path: PathBuf,
}

pub fn resolve_storage(global_flag: bool, cwd: &Path) -> Result<StorageContext> {
    let tasks_path = if global_flag {
        global_tasks_path()?
    } else if let Some(repo_root) = git::repo_root_from(cwd)? {
        repo_root.join(".shipflow").join("tasks.json")
    } else {
        global_tasks_path()?
    };

    let mode = if global_flag || git::repo_root_from(cwd)?.is_none() {
        StorageMode::Global(tasks_path.clone())
    } else {
        StorageMode::Repo(tasks_path.clone())
    };

    Ok(StorageContext { mode, tasks_path })
}

fn global_tasks_path() -> Result<PathBuf> {
    let proj_dirs =
        ProjectDirs::from("com", "shipflow", "shipflow").ok_or_else(|| ShipflowError::Storage {
            path: PathBuf::from("~/.config/shipflow"),
            message: "could not resolve config directory".to_owned(),
        })?;
    Ok(proj_dirs.config_dir().join("tasks.json"))
}

pub struct TaskStore {
    path: PathBuf,
    file: TaskFile,
}

impl TaskStore {
    pub fn open(ctx: &StorageContext) -> Result<Self> {
        let path = ctx.tasks_path.clone();
        let file = load_file(&path)?;
        Ok(Self { path, file })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn tasks(&self) -> &[Task] {
        &self.file.tasks
    }

    pub fn add(&mut self, task: Task) -> Result<Task> {
        self.file.tasks.push(task);
        self.save()?;
        self.file
            .tasks
            .last()
            .cloned()
            .ok_or_else(|| ShipflowError::Storage {
                path: self.path.clone(),
                message: "failed to read back added task".to_owned(),
            })
    }

    pub fn find(&self, query: &str) -> Result<&Task> {
        resolve_task(&self.file.tasks, query)
    }

    pub fn mark_done(
        &mut self,
        query: &str,
        linked_commit: Option<crate::task::LinkedCommit>,
    ) -> Result<Task> {
        let id = {
            let task = resolve_task(&self.file.tasks, query)?;
            task.id.clone()
        };

        let index = self
            .file
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| ShipflowError::TaskNotFound(id.clone()))?;

        self.file.tasks[index].mark_done(linked_commit);
        self.save()?;
        Ok(self.file.tasks[index].clone())
    }

    pub fn save(&self) -> Result<()> {
        save_file(&self.path, &self.file)
    }
}

fn load_file(path: &Path) -> Result<TaskFile> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let empty = TaskFile::empty();
        save_file(path, &empty)?;
        return Ok(empty);
    }

    let data = fs::read_to_string(path)?;
    let mut file: TaskFile = serde_json::from_str(&data)?;
    migrate(&mut file)?;
    Ok(file)
}

fn save_file(path: &Path, file: &TaskFile) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut sorted = file.clone();
    sorted.tasks.sort_by_key(|t| t.created_at);

    let tmp_path = path.with_extension("json.tmp");
    let json = serde_json::to_string_pretty(&sorted)?;
    fs::write(&tmp_path, json)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn migrate(file: &mut TaskFile) -> Result<()> {
    if file.version == 0 {
        file.version = SCHEMA_VERSION;
    }
    if file.version > SCHEMA_VERSION {
        return Err(ShipflowError::Storage {
            path: PathBuf::from("tasks.json"),
            message: format!(
                "unsupported schema version {} (max supported: {SCHEMA_VERSION})",
                file.version
            ),
        });
    }
    Ok(())
}

pub fn filter_tasks<'a>(
    tasks: &'a [Task],
    status: Option<TaskStatus>,
    tags: &[String],
) -> Vec<&'a Task> {
    tasks
        .iter()
        .filter(|t| status.is_none_or(|s| t.status == s))
        .filter(|t| crate::task::tag_matches(&t.tags, tags))
        .collect()
}

pub fn count_by_status(tasks: &[Task]) -> (usize, usize) {
    let open = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Open)
        .count();
    let done = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Done)
        .count();
    (open, done)
}

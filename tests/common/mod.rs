use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn shipflow_cmd() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("shipflow").unwrap()
}

pub fn init_git_repo(dir: &Path) {
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .status()
        .expect("git init");
    std::process::Command::new("git")
        .args(["config", "user.email", "test@shipflow.dev"])
        .current_dir(dir)
        .status()
        .expect("git config email");
    std::process::Command::new("git")
        .args(["config", "user.name", "shipflow test"])
        .current_dir(dir)
        .status()
        .expect("git config name");
}

/// Expected global `tasks.json` path when env vars from [`global_storage_env`] are set.
#[allow(dead_code)]
pub fn global_storage_path(base: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        base.join("AppData")
            .join("Roaming")
            .join("shipflow")
            .join("tasks.json")
    }
    #[cfg(not(windows))]
    {
        base.join(".config").join("shipflow").join("tasks.json")
    }
}

/// Env vars that redirect shipflow global storage into `base` (cross-platform).
pub fn global_storage_env(base: &Path) -> HashMap<String, String> {
    let mut env = HashMap::new();

    #[cfg(windows)]
    {
        let appdata = base.join("AppData").join("Roaming");
        fs::create_dir_all(&appdata).expect("create APPDATA dir");
        env.insert("USERPROFILE".into(), path_to_env(base));
        env.insert("APPDATA".into(), path_to_env(&appdata));
    }

    #[cfg(not(windows))]
    {
        let config = base.join(".config");
        fs::create_dir_all(&config).expect("create config dir");
        env.insert("HOME".into(), path_to_env(base));
        env.insert("XDG_CONFIG_HOME".into(), path_to_env(&config));
    }

    env
}

#[allow(dead_code)]
pub fn repo_storage_path(repo: &Path) -> PathBuf {
    repo.join(".shipflow").join("tasks.json")
}

#[allow(dead_code)]
pub fn write_sample_tasks(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(
        path,
        r#"{
  "version": 1,
  "tasks": []
}"#,
    )
    .expect("write tasks");
}

fn path_to_env(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

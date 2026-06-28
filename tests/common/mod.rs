use std::fs;
use std::path::{Path, PathBuf};

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

pub fn global_storage_path(home: &Path) -> PathBuf {
    home.join(".config").join("shipflow").join("tasks.json")
}

pub fn repo_storage_path(repo: &Path) -> PathBuf {
    repo.join(".shipflow").join("tasks.json")
}

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

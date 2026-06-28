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
    base.join(".shipflow-global").join("tasks.json")
}

/// Env vars that redirect shipflow global storage into `base` (cross-platform).
#[allow(dead_code)]
pub fn global_storage_env(base: &Path) -> HashMap<String, String> {
    let config_dir = base.join(".shipflow-global");
    fs::create_dir_all(&config_dir).expect("create global config dir");

    let mut env = HashMap::new();
    env.insert(
        "SHIPFLOW_CONFIG_DIR".into(),
        config_dir.to_string_lossy().into_owned(),
    );
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

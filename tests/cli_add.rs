mod common;

use predicates::prelude::*;

#[test]
fn add_creates_task_in_repo_mode() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Ship auth refactor", "--tags", "rust,auth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added"))
        .stdout(predicate::str::contains("Ship auth refactor"));

    assert!(common::repo_storage_path(repo).exists());
}

#[test]
fn add_global_flag_uses_global_storage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    let home = repo.join("home");
    let config = home.join(".config");
    std::fs::create_dir_all(&config).unwrap();

    common::shipflow_cmd()
        .current_dir(repo)
        .env("HOME", &home)
        .env("XDG_CONFIG_HOME", &config)
        .args(["add", "--global", "Global task"])
        .assert()
        .success();

    assert!(common::global_storage_path(&home).exists());
}

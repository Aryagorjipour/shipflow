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
    std::fs::create_dir_all(&home).unwrap();

    let mut cmd = common::shipflow_cmd();
    cmd.current_dir(repo);
    for (key, value) in common::global_storage_env(&home) {
        cmd.env(key, value);
    }

    cmd.args(["add", "--global", "Global task"])
        .assert()
        .success();

    assert!(common::global_storage_path(&home).exists());
}

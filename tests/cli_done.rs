mod common;

use predicates::prelude::*;

#[test]
fn done_marks_task_by_partial_title() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Fix parser edge case"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["done", "parser", "--no-link"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Done"))
        .stdout(predicate::str::contains("Fix parser edge case"));

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["list", "--status", "done"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fix parser edge case"));
}

#[test]
fn done_errors_on_missing_task() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["done", "missing", "--no-link"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("task not found"));
}

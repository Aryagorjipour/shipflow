mod common;

use predicates::prelude::*;

#[test]
fn list_shows_added_tasks() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "First task", "--tags", "a"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Second task"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["list", "--status", "open"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First task"))
        .stdout(predicate::str::contains("Second task"));
}

#[test]
fn list_filters_done_tasks() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Todo item"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Done item"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["done", "Done item", "--no-link"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["list", "--status", "done"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Done item"))
        .stdout(predicate::str::contains("Todo item").not());
}

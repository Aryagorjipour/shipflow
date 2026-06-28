mod common;

use predicates::prelude::*;

#[test]
fn report_renders_markdown() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Ship CLI polish", "--tags", "rust"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["done", "CLI polish", "--no-link"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["report", "week", "--format", "md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# What I shipped"))
        .stdout(predicate::str::contains("Ship CLI polish"));
}

#[test]
fn status_shows_counts() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["add", "Open one"])
        .assert()
        .success();

    common::shipflow_cmd()
        .current_dir(repo)
        .args(["status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("tasks:"))
        .stdout(predicate::str::contains("open"));
}

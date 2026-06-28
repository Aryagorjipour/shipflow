mod common;

use std::fs;

use shipflow::storage::{TaskStore, resolve_storage};
use shipflow::task::Task;

#[test]
fn uses_repo_storage_inside_git_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    let ctx = resolve_storage(false, repo).unwrap();
    assert!(ctx.tasks_path.ends_with(".shipflow/tasks.json"));
    assert_eq!(ctx.mode.label(), "repo");

    let mut store = TaskStore::open(&ctx).unwrap();
    store.add(Task::new("Repo task", vec![], None)).unwrap();
    assert!(common::repo_storage_path(repo).exists());
}

#[test]
fn uses_global_storage_outside_git_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.path();

    let ctx = resolve_storage(false, dir).unwrap();
    assert_eq!(ctx.mode.label(), "global");
}

#[test]
fn global_flag_overrides_repo_storage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    let ctx = resolve_storage(true, repo).unwrap();
    assert_eq!(ctx.mode.label(), "global");
}

#[test]
fn round_trip_task_persistence() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo = temp.path();
    common::init_git_repo(repo);

    let ctx = resolve_storage(false, repo).unwrap();
    let mut store = TaskStore::open(&ctx).unwrap();
    let created = store
        .add(Task::new(
            "Persist me",
            vec!["rust".to_owned()],
            Some("note".to_owned()),
        ))
        .unwrap();

    let store2 = TaskStore::open(&ctx).unwrap();
    let found = store2.find(&created.id).unwrap();
    assert_eq!(found.title, "Persist me");
    assert_eq!(found.tags, vec!["rust".to_owned()]);
    assert_eq!(found.note.as_deref(), Some("note"));

    let raw = fs::read_to_string(ctx.tasks_path).unwrap();
    assert!(raw.contains("Persist me"));
}

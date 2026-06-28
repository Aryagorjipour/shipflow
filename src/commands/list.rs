use comfy_table::{Cell, Table, presets::UTF8_FULL};

use crate::cli::ListStatus;
use crate::error::Result;
use crate::storage::{TaskStore, filter_tasks, resolve_storage};
use crate::task::TaskStatus;
use crate::utils::{is_color_enabled, with_color};

pub fn run(status: ListStatus, tags: Vec<String>, global: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let ctx = resolve_storage(global, &cwd)?;
    let store = TaskStore::open(&ctx)?;

    let status_filter = match status {
        ListStatus::Open => Some(TaskStatus::Open),
        ListStatus::Done => Some(TaskStatus::Done),
        ListStatus::All => None,
    };

    let tasks = filter_tasks(store.tasks(), status_filter, &tags);
    let color = with_color(is_color_enabled());

    if tasks.is_empty() {
        println!("{}", color.dim("No tasks found."));
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Status", "Title", "Tags", "Created"]);

    for task in tasks {
        let status_str = match task.status {
            TaskStatus::Open => "open",
            TaskStatus::Done => "done",
        };
        let tags_str = if task.tags.is_empty() {
            "-".to_owned()
        } else {
            task.tags.join(", ")
        };

        table.add_row(vec![
            Cell::new(&task.id[..8.min(task.id.len())]),
            Cell::new(status_str),
            Cell::new(&task.title),
            Cell::new(tags_str),
            Cell::new(task.created_at.format("%Y-%m-%d").to_string()),
        ]);
    }

    println!("{table}");
    Ok(())
}

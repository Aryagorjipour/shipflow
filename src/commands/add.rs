use crate::error::Result;
use crate::storage::{TaskStore, resolve_storage};
use crate::task::Task;
use crate::utils::{self, is_color_enabled, with_color};

pub fn run(title: String, tags: Vec<String>, note: Option<String>, global: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let ctx = resolve_storage(global, &cwd)?;
    let mut store = TaskStore::open(&ctx)?;

    let task = Task::new(title, tags, note);
    let created = store.add(task)?;

    let color = with_color(is_color_enabled());
    utils::print_success(&format!(
        "Added {} — {}",
        color.id(&created.id[..8.min(created.id.len())]),
        color.highlight(&created.title)
    ))?;

    Ok(())
}

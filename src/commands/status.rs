use crate::error::Result;
use crate::git;
use crate::storage::{TaskStore, count_by_status, resolve_storage};
use crate::utils::{is_color_enabled, with_color};

pub fn run(global: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let ctx = resolve_storage(global, &cwd)?;
    let store = TaskStore::open(&ctx)?;
    let git_ctx = git::detect(&cwd);
    let (open, done) = count_by_status(store.tasks());
    let color = with_color(is_color_enabled());

    println!("{}", color.highlight("shipflow status"));
    println!(
        "{}",
        color.dim(&format!(
            "storage: {} ({})",
            store.path().display(),
            ctx.mode.label()
        ))
    );
    println!("tasks: {} open · {} done", open, done);

    if !git::git_available() {
        println!("{}", color.dim("git: not available"));
        return Ok(());
    }

    if git_ctx.is_repo {
        let branch = git_ctx.branch.as_deref().unwrap_or("(detached)");
        println!(
            "git: {} @ {}",
            branch,
            git_ctx
                .repo_root
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        );

        if let Some(root) = git_ctx.repo_root.as_ref() {
            let commits = git::recent_commits(root, 3)?;
            if !commits.is_empty() {
                println!("{}", color.info("recent commits:"));
                for c in commits {
                    println!("  {} {}", c.sha, c.message);
                }
            }
        }
    } else {
        println!("{}", color.dim("git: not in a repository"));
    }

    Ok(())
}

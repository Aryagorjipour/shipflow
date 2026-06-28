use std::io::IsTerminal;

use crate::error::{Result, ShipflowError};
use crate::git::{self, CommitSummary};
use crate::storage::{TaskStore, resolve_storage};
use crate::task::LinkedCommit;
use crate::utils::{self, is_color_enabled, read_line, with_color};

pub fn run(query: String, commit: Option<String>, no_link: bool, global: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let ctx = resolve_storage(global, &cwd)?;
    let git_ctx = git::detect(&cwd);
    let mut store = TaskStore::open(&ctx)?;

    let linked = resolve_linked_commit(&git_ctx, commit, no_link)?;

    let done = store.mark_done(&query, linked)?;
    let color = with_color(is_color_enabled());

    let mut msg = format!(
        "Done {} — {}",
        color.id(&done.id[..8.min(done.id.len())]),
        color.highlight(&done.title)
    );

    if let Some(c) = &done.linked_commit {
        msg.push_str(&format!("\n  linked: {} {}", c.sha, c.message));
    }

    utils::print_success(&msg)?;
    Ok(())
}

fn resolve_linked_commit(
    git_ctx: &git::GitContext,
    commit_flag: Option<String>,
    no_link: bool,
) -> Result<Option<LinkedCommit>> {
    if no_link {
        return Ok(None);
    }

    let Some(repo_root) = git_ctx.repo_root.as_ref() else {
        return Ok(None);
    };

    if let Some(sha) = commit_flag {
        if !git::validate_commit(repo_root, &sha)? {
            return Err(ShipflowError::InvalidCommit(sha));
        }
        return git::linked_commit_from_sha(repo_root, &sha, git_ctx.branch.as_deref());
    }

    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return Ok(None);
    }

    let commits = git::recent_commits(repo_root, 5)?;
    if commits.is_empty() {
        return Ok(None);
    }

    prompt_commit_selection(repo_root, &commits, git_ctx.branch.as_deref())
}

fn prompt_commit_selection(
    repo_root: &std::path::Path,
    commits: &[CommitSummary],
    branch: Option<&str>,
) -> Result<Option<LinkedCommit>> {
    let color = with_color(is_color_enabled());
    println!("{}", color.info("Recent commits:"));
    println!("{}", color.dim("  0) Skip"));

    for (i, c) in commits.iter().enumerate() {
        println!("  {}) {} {}", i + 1, c.sha, c.message);
    }

    let input = read_line("Link commit [0-5]: ")?;
    if input.is_empty() || input == "0" {
        return Ok(None);
    }

    let choice: usize = input
        .parse()
        .map_err(|_| ShipflowError::InvalidCommit(input.clone()))?;

    if choice == 0 || choice > commits.len() {
        return Ok(None);
    }

    let selected = &commits[choice - 1];
    git::linked_commit_from_sha(repo_root, &selected.sha, branch)
}

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::Result;
use crate::task::LinkedCommit;

#[derive(Debug, Clone)]
pub struct GitContext {
    pub is_repo: bool,
    pub repo_root: Option<PathBuf>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CommitSummary {
    pub sha: String,
    pub message: String,
}

pub fn detect(cwd: &Path) -> GitContext {
    let repo_root = repo_root_from(cwd).ok().flatten();
    let branch = repo_root
        .as_ref()
        .and_then(|root| current_branch(root).ok().flatten());

    GitContext {
        is_repo: repo_root.is_some(),
        repo_root,
        branch,
    }
}

pub fn repo_root_from(cwd: &Path) -> Result<Option<PathBuf>> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(cwd)
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let root = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if root.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PathBuf::from(root)))
    }
}

fn current_branch(repo_root: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if branch.is_empty() || branch == "HEAD" {
        Ok(None)
    } else {
        Ok(Some(branch))
    }
}

pub fn recent_commits(repo_root: &Path, n: usize) -> Result<Vec<CommitSummary>> {
    let output = Command::new("git")
        .args(["log", &format!("-{n}"), "--pretty=format:%h|%s"])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let commits = text
        .lines()
        .filter_map(|line| {
            let (sha, message) = line.split_once('|')?;
            Some(CommitSummary {
                sha: sha.to_owned(),
                message: message.to_owned(),
            })
        })
        .collect();

    Ok(commits)
}

pub fn validate_commit(repo_root: &Path, sha: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["cat-file", "-e", &format!("{sha}^{{commit}}")])
        .current_dir(repo_root)
        .output()?;

    Ok(output.status.success())
}

pub fn commit_message(repo_root: &Path, sha: &str) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["log", "-1", "--pretty=format:%s", sha])
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let message = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if message.is_empty() {
        Ok(None)
    } else {
        Ok(Some(message))
    }
}

pub fn linked_commit_from_sha(
    repo_root: &Path,
    sha: &str,
    branch: Option<&str>,
) -> Result<Option<LinkedCommit>> {
    if !validate_commit(repo_root, sha)? {
        return Ok(None);
    }

    let message = commit_message(repo_root, sha)?.unwrap_or_else(|| "(no message)".to_owned());
    Ok(Some(LinkedCommit {
        sha: sha.to_owned(),
        message,
        branch: branch.map(ToOwned::to_owned),
    }))
}

pub fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

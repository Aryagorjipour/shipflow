use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::error::{Result, ShipflowError};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedCommit {
    pub sha: String,
    pub message: String,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_commit: Option<LinkedCommit>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskFile {
    pub version: u32,
    pub tasks: Vec<Task>,
}

impl TaskFile {
    pub fn empty() -> Self {
        Self {
            version: SCHEMA_VERSION,
            tasks: Vec::new(),
        }
    }
}

impl Task {
    pub fn new(title: impl Into<String>, tags: Vec<String>, note: Option<String>) -> Self {
        Self {
            id: Ulid::new().to_string(),
            title: title.into(),
            tags,
            note,
            status: TaskStatus::Open,
            created_at: Utc::now(),
            completed_at: None,
            linked_commit: None,
        }
    }

    pub fn mark_done(&mut self, linked_commit: Option<LinkedCommit>) {
        self.status = TaskStatus::Done;
        self.completed_at = Some(Utc::now());
        self.linked_commit = linked_commit;
    }
}

pub fn parse_tags(raw: Option<&str>) -> Vec<String> {
    raw.map(|s| {
        s.split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    })
    .unwrap_or_default()
}

pub fn tag_matches(task_tags: &[String], filter_tags: &[String]) -> bool {
    if filter_tags.is_empty() {
        return true;
    }
    filter_tags
        .iter()
        .all(|filter| task_tags.iter().any(|tag| tag.eq_ignore_ascii_case(filter)))
}

pub fn find_by_id_prefix<'a>(tasks: &'a [Task], query: &str) -> Result<&'a Task> {
    let matches: Vec<_> = tasks.iter().filter(|t| t.id.starts_with(query)).collect();

    match matches.len() {
        0 => Err(ShipflowError::TaskNotFound(query.to_owned())),
        1 => Ok(matches[0]),
        _ => Err(ShipflowError::AmbiguousMatch {
            query: query.to_owned(),
            matches: format_matches(&matches),
        }),
    }
}

pub fn find_by_partial_title<'a>(tasks: &'a [Task], query: &str) -> Result<&'a Task> {
    let query_lower = query.to_lowercase();
    let matches: Vec<_> = tasks
        .iter()
        .filter(|t| t.title.to_lowercase().contains(&query_lower))
        .collect();

    match matches.len() {
        0 => Err(ShipflowError::TaskNotFound(query.to_owned())),
        1 => Ok(matches[0]),
        _ => Err(ShipflowError::AmbiguousMatch {
            query: query.to_owned(),
            matches: format_matches(&matches),
        }),
    }
}

pub fn resolve_task<'a>(tasks: &'a [Task], query: &str) -> Result<&'a Task> {
    if tasks.iter().any(|t| t.id.starts_with(query)) {
        find_by_id_prefix(tasks, query)
    } else {
        find_by_partial_title(tasks, query)
    }
}

fn format_matches(tasks: &[&Task]) -> String {
    tasks
        .iter()
        .map(|t| format!("  {} — {}", t.id, t.title))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tags_splits_and_trims() {
        assert_eq!(
            parse_tags(Some(" rust, cli ,,tui ")),
            vec!["rust", "cli", "tui"]
        );
    }

    #[test]
    fn tag_matches_is_case_insensitive_and_and_semantics() {
        let tags = vec!["Rust".to_owned(), "CLI".to_owned()];
        assert!(tag_matches(&tags, &["rust".to_owned()]));
        assert!(tag_matches(&tags, &["rust".to_owned(), "cli".to_owned()]));
        assert!(!tag_matches(&tags, &["rust".to_owned(), "web".to_owned()]));
    }
}

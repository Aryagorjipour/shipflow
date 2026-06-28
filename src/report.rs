use chrono::{DateTime, Duration, Utc};
use comfy_table::{Cell, Table, presets::UTF8_FULL};

use crate::task::{Task, TaskStatus};
use crate::utils::ColorMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportPeriod {
    Today,
    Week,
    Month,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Text,
    Markdown,
}

impl ReportPeriod {
    pub fn start(&self, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Self::All => None,
            Self::Today => Some(now.date_naive().and_hms_opt(0, 0, 0)?.and_utc()),
            Self::Week => Some(now - Duration::days(7)),
            Self::Month => Some(now - Duration::days(30)),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::Week => "this week",
            Self::Month => "this month",
            Self::All => "all time",
        }
    }
}

pub struct ReportStats {
    pub shipped: usize,
    pub linked_commits: usize,
    pub top_tags: Vec<(String, usize)>,
}

pub fn shipped_tasks(tasks: &[Task], period: ReportPeriod) -> Vec<&Task> {
    let now = Utc::now();
    let start = period.start(now);

    let mut done: Vec<_> = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Done)
        .filter(|t| {
            start.is_none_or(|s| {
                t.completed_at
                    .is_some_and(|completed| completed >= s && completed <= now)
            })
        })
        .collect();

    done.sort_by_key(|t| t.completed_at);
    done.reverse();
    done
}

pub fn compute_stats(tasks: &[&Task]) -> ReportStats {
    let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for task in tasks {
        for tag in &task.tags {
            *tag_counts.entry(tag.clone()).or_default() += 1;
        }
    }

    let mut top_tags: Vec<_> = tag_counts.into_iter().collect();
    top_tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    top_tags.truncate(5);

    let linked_commits = tasks.iter().filter(|t| t.linked_commit.is_some()).count();

    ReportStats {
        shipped: tasks.len(),
        linked_commits,
        top_tags,
    }
}

pub fn render_text(
    shipped: &[&Task],
    stats: &ReportStats,
    period: ReportPeriod,
    color: ColorMode,
) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "{}\n",
        color.highlight(&format!("What I shipped — {}", period.label()))
    ));
    out.push_str(&format!(
        "{}\n\n",
        color.dim(&format!(
            "{} tasks · {} linked commits",
            stats.shipped, stats.linked_commits
        ))
    ));

    if !stats.top_tags.is_empty() {
        let tags: Vec<_> = stats
            .top_tags
            .iter()
            .map(|(tag, count)| format!("{tag} ({count})"))
            .collect();
        out.push_str(&format!("Focus: {}\n\n", color.info(&tags.join(", "))));
    }

    if shipped.is_empty() {
        out.push_str(&format!(
            "{}\n",
            color.dim("No shipped tasks in this period.")
        ));
        return out;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Title", "Tags", "Commit"]);

    for task in shipped {
        let tags = if task.tags.is_empty() {
            "-".to_owned()
        } else {
            task.tags.join(", ")
        };
        let commit = task
            .linked_commit
            .as_ref()
            .map(|c| format!("{} {}", c.sha, c.message))
            .unwrap_or_else(|| "-".to_owned());

        table.add_row(vec![
            Cell::new(&task.id[..8.min(task.id.len())]),
            Cell::new(&task.title),
            Cell::new(tags),
            Cell::new(commit),
        ]);
    }

    out.push_str(&table.to_string());
    out.push('\n');
    out
}

pub fn render_markdown(shipped: &[&Task], stats: &ReportStats, period: ReportPeriod) -> String {
    let now = Utc::now();
    let mut out = String::new();

    out.push_str(&format!("# What I shipped — {}\n\n", period.label()));
    out.push_str(&format!(
        "_Generated {}_\n\n",
        now.format("%Y-%m-%d %H:%M UTC")
    ));
    out.push_str("## Summary\n\n");
    out.push_str(&format!("- **Shipped:** {}\n", stats.shipped));
    out.push_str(&format!("- **Linked commits:** {}\n", stats.linked_commits));

    if !stats.top_tags.is_empty() {
        out.push_str("- **Focus areas:** ");
        let tags: Vec<_> = stats
            .top_tags
            .iter()
            .map(|(tag, count)| format!("`{tag}` ({count})"))
            .collect();
        out.push_str(&tags.join(", "));
        out.push('\n');
    }

    out.push_str("\n## Shipped\n\n");

    if shipped.is_empty() {
        out.push_str("_No shipped tasks in this period._\n");
        return out;
    }

    for task in shipped {
        let completed = task
            .completed_at
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "?".to_owned());

        out.push_str(&format!("### {} ({})\n\n", task.title, completed));

        if !task.tags.is_empty() {
            out.push_str(&format!(
                "**Tags:** {}\n\n",
                task.tags
                    .iter()
                    .map(|t| format!("`{t}`"))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }

        if let Some(note) = &task.note {
            out.push_str(&format!("{note}\n\n"));
        }

        if let Some(commit) = &task.linked_commit {
            out.push_str(&format!(
                "**Commit:** `{}` — {}\n\n",
                commit.sha, commit.message
            ));
        }
    }

    out
}

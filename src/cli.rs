use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "shipflow",
    version,
    about = "Track lightweight intentions and celebrate what you ship",
    long_about = "shipflow is a local-first, git-aware CLI for tracking what you intend to ship \
                  and generating reflective reports of what you actually shipped.",
    after_help = "EXAMPLES:
  shipflow add \"Fix login bug\" --tags rust,auth
  shipflow list --status open
  shipflow done fix-login
  shipflow report week --format md
  shipflow status
  shipflow board
  shipflow completions fish > ~/.config/fish/completions/shipflow.fish"
)]
pub struct Cli {
    /// Enable verbose logging (sets RUST_LOG=shipflow=debug)
    #[arg(long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new task to track
    Add {
        /// Task title
        title: String,

        /// Comma-separated tags
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Optional note
        #[arg(long)]
        note: Option<String>,

        /// Store in global config instead of current repo
        #[arg(long)]
        global: bool,
    },

    /// List tasks
    List {
        /// Filter by status
        #[arg(long, value_enum, default_value = "all")]
        status: ListStatus,

        /// Filter by tags (AND semantics)
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Use global storage
        #[arg(long)]
        global: bool,
    },

    /// Mark a task as done
    Done {
        /// Task ID prefix or partial title
        query: String,

        /// Link a specific git commit SHA
        #[arg(long)]
        commit: Option<String>,

        /// Skip git commit linking
        #[arg(long)]
        no_link: bool,

        /// Use global storage
        #[arg(long)]
        global: bool,
    },

    /// Generate a \"What I shipped\" report
    Report {
        /// Time period
        #[arg(value_enum, default_value = "week")]
        period: ReportPeriodArg,

        /// Output format
        #[arg(long, value_enum, default_value = "text")]
        format: ReportFormatArg,

        /// Use global storage
        #[arg(long)]
        global: bool,
    },

    /// Show overview and git context
    Status {
        /// Use global storage
        #[arg(long)]
        global: bool,
    },

    /// Interactive kanban board (requires `tui` feature)
    #[cfg(feature = "tui")]
    Board {
        /// Use global storage
        #[arg(long)]
        global: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ListStatus {
    Open,
    Done,
    All,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReportPeriodArg {
    Today,
    Week,
    Month,
    All,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReportFormatArg {
    Text,
    Md,
}

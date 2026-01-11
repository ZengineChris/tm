use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "tm",
    version,
    about = "Task Manager - Git worktree-based task management",
    long_about = "A CLI tool for managing tasks associated with git worktrees. \
                  Follows Unix philosophy - integrates with sesh for tmux and pm for project management."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Level {
    Feature,
    Fix,
    Chore,
    Docs,
    Refactor,
    Test,
    Perf,
    Style,
    Ci,
}

impl Level {
    /// Convert to lowercase string for branch names and paths
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Feature => "feature",
            Level::Fix => "fix",
            Level::Chore => "chore",
            Level::Docs => "docs",
            Level::Refactor => "refactor",
            Level::Test => "test",
            Level::Perf => "perf",
            Level::Style => "style",
            Level::Ci => "ci",
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Project name
        project: String,

        /// Path to main repository (e.g., ~/projects/myapp/main)
        main_repo_path: PathBuf,

        /// Task level (feature, fix, chore, etc.)
        #[arg(short, long, value_enum)]
        level: Level,

        /// Task ID/reference (e.g., JIRA-123)
        #[arg(short, long)]
        id: String,

        /// Task name (will be converted to snake_case)
        #[arg(short, long)]
        name: String,

        /// Task description
        #[arg(short, long)]
        description: Option<String>,

        /// Remote URL for future integration
        #[arg(long)]
        remote_url: Option<String>,

        /// API URL for future integration
        #[arg(long)]
        api_url: Option<String>,
    },

    /// List tasks
    List {
        /// Filter by project name
        #[arg(short, long)]
        project: Option<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },

    /// Remove a task
    Remove {
        /// Project name
        project: String,

        /// Task title
        title: String,

        /// Also remove the git worktree
        #[arg(short = 'w', long)]
        remove_worktree: bool,

        /// Force removal even if worktree has uncommitted changes
        #[arg(short, long, requires = "remove_worktree")]
        force: bool,
    },

    /// Switch to a task (outputs worktree path for shell integration)
    Switch {
        /// Project name
        project: String,

        /// Task title
        title: String,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    /// Table format with columns
    Table,
    /// Simple list format
    Simple,
    /// JSON format for scripting
    Json,
}

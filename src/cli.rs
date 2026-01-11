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

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Project name
        project: String,

        /// Task title (unique within project)
        title: String,

        /// Worktree path
        #[arg(short = 'p', long)]
        worktree_path: PathBuf,

        /// Task description
        #[arg(short, long)]
        description: Option<String>,

        /// Reference ID (e.g., JIRA-123)
        #[arg(short, long)]
        reference: Option<String>,

        /// Remote URL for future integration
        #[arg(long)]
        remote_url: Option<String>,

        /// API URL for future integration
        #[arg(long)]
        api_url: Option<String>,

        /// Create the git worktree if it doesn't exist
        #[arg(short = 'c', long)]
        create_worktree: bool,

        /// Base branch for worktree creation (requires --create-worktree)
        #[arg(short = 'b', long, requires = "create_worktree")]
        base_branch: Option<String>,

        /// Main repository path for worktree creation (requires --create-worktree)
        #[arg(short = 'm', long, requires = "create_worktree")]
        main_repo: Option<PathBuf>,
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

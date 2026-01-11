use std::path::PathBuf;
use thiserror::Error;

pub type TmResult<T> = Result<T, TmError>;

#[derive(Error, Debug)]
pub enum TmError {
    #[error("Task '{title}' already exists in project '{project}'")]
    DuplicateTask { project: String, title: String },

    #[error("Project '{project}' not found")]
    ProjectNotFound { project: String },

    #[error("Task '{title}' not found in project '{project}'")]
    TaskNotFound { project: String, title: String },

    #[error("Worktree path does not exist: {path}")]
    WorktreePathNotFound { path: PathBuf },

    #[error("Path is not a valid git worktree: {path}")]
    InvalidWorktree { path: PathBuf },

    #[error("Failed to create worktree at {path}: {reason}")]
    WorktreeCreationFailed { path: PathBuf, reason: String },

    #[error("Failed to remove worktree at {path}: {reason}")]
    WorktreeRemovalFailed { path: PathBuf, reason: String },

    #[error("Worktree has uncommitted changes at {path}. Use --force to remove anyway.")]
    WorktreeHasChanges { path: PathBuf },

    #[error("Git repository not found at {path}")]
    GitRepoNotFound { path: PathBuf },

    #[error("Invalid main repository path: {path}")]
    InvalidMainRepoPath { path: PathBuf },

    #[error("Worktree already exists at {path}")]
    WorktreeAlreadyExists { path: PathBuf },

    #[error("Invalid input for {field}: {reason}")]
    InvalidInput { field: String, reason: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),
}

impl TmError {
    /// Helper for user-friendly error messages
    pub fn user_message(&self) -> String {
        match self {
            TmError::DuplicateTask { project, title } => {
                format!(
                    "A task named '{}' already exists in project '{}'. Please use a different title.",
                    title, project
                )
            }
            TmError::TaskNotFound { project, title } => {
                format!(
                    "Could not find task '{}' in project '{}'. Use 'tm list' to see available tasks.",
                    title, project
                )
            }
            TmError::WorktreeHasChanges { path } => {
                format!(
                    "The worktree at '{}' has uncommitted changes.\n\
                    Either commit or stash your changes, or use --force to remove anyway.",
                    path.display()
                )
            }
            TmError::InvalidMainRepoPath { path } => {
                format!(
                    "The main repository path '{}' is invalid.\n\
                    Please provide a path to your main branch directory (e.g., ~/projects/myapp/main).",
                    path.display()
                )
            }
            TmError::WorktreeAlreadyExists { path } => {
                format!(
                    "A worktree already exists at '{}'.\n\
                    Please remove it first or use a different name/id.",
                    path.display()
                )
            }
            TmError::InvalidInput { field, reason } => {
                format!("Invalid {}: {}", field, reason)
            }
            _ => self.to_string(),
        }
    }
}

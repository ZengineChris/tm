use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    /// Task title - used as unique identifier within a project
    pub title: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Path to the git worktree
    pub worktree_path: PathBuf,

    /// Reference ID for commits (e.g., JIRA-123)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Remote URL for future GitHub/Jira integration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,

    /// API URL for future GitHub/Jira integration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
}

impl Task {
    /// Create a new task with required fields
    pub fn new(title: String, worktree_path: PathBuf) -> Self {
        Self {
            title,
            description: None,
            worktree_path,
            reference: None,
            remote_url: None,
            api_url: None,
        }
    }

    /// Builder pattern for optional fields
    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn with_reference(mut self, reference: String) -> Self {
        self.reference = Some(reference);
        self
    }

    pub fn with_remote_url(mut self, url: String) -> Self {
        self.remote_url = Some(url);
        self
    }

    pub fn with_api_url(mut self, url: String) -> Self {
        self.api_url = Some(url);
        self
    }
}

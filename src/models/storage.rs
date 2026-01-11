use crate::error::TmResult;
use crate::models::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskStorage {
    /// Tasks grouped by project name
    #[serde(default)]
    pub projects: HashMap<String, Vec<Task>>,
}

impl TaskStorage {
    /// Create new empty storage
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    /// Load storage from TOML file
    pub fn load(path: &PathBuf) -> TmResult<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = std::fs::read_to_string(path)?;
        let storage: TaskStorage = toml::from_str(&content)?;
        Ok(storage)
    }

    /// Save storage to TOML file
    pub fn save(&self, path: &PathBuf) -> TmResult<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add a task to a project
    pub fn add_task(&mut self, project: String, task: Task) -> TmResult<()> {
        let tasks = self.projects.entry(project.clone()).or_default();

        // Check for duplicate title
        if tasks.iter().any(|t| t.title == task.title) {
            return Err(crate::error::TmError::DuplicateTask {
                project,
                title: task.title.clone(),
            });
        }

        tasks.push(task);
        Ok(())
    }

    /// Remove a task from a project
    pub fn remove_task(&mut self, project: &str, title: &str) -> TmResult<Task> {
        let tasks = self
            .projects
            .get_mut(project)
            .ok_or_else(|| crate::error::TmError::ProjectNotFound {
                project: project.to_string(),
            })?;

        let index = tasks
            .iter()
            .position(|t| t.title == title)
            .ok_or_else(|| crate::error::TmError::TaskNotFound {
                project: project.to_string(),
                title: title.to_string(),
            })?;

        let task = tasks.remove(index);

        // Remove empty projects
        if tasks.is_empty() {
            self.projects.remove(project);
        }

        Ok(task)
    }

    /// Get a task by project and title
    pub fn get_task(&self, project: &str, title: &str) -> TmResult<&Task> {
        self.projects
            .get(project)
            .and_then(|tasks| tasks.iter().find(|t| t.title == title))
            .ok_or_else(|| crate::error::TmError::TaskNotFound {
                project: project.to_string(),
                title: title.to_string(),
            })
    }

    /// List all tasks, optionally filtered by project
    pub fn list_tasks(&self, project_filter: Option<&str>) -> Vec<(&str, &Task)> {
        let mut result = Vec::new();

        for (project, tasks) in &self.projects {
            if let Some(filter) = project_filter {
                if project != filter {
                    continue;
                }
            }

            for task in tasks {
                result.push((project.as_str(), task));
            }
        }

        result
    }
}

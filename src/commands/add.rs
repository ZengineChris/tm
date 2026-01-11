use crate::cli::Level;
use crate::config::get_tasks_file_path;
use crate::error::{TmError, TmResult};
use crate::git;
use crate::models::{storage::TaskStorage, task::Task};
use std::path::PathBuf;

/// Validate input parameters
fn validate_inputs(id: &str, name: &str) -> TmResult<()> {
    // Check for empty name after conversion
    let name_snake = git::to_snake_case(name);
    if name_snake.is_empty() {
        return Err(TmError::InvalidInput {
            field: "name".to_string(),
            reason: "Name must contain alphanumeric characters".to_string(),
        });
    }

    // Check for empty ID
    if id.trim().is_empty() {
        return Err(TmError::InvalidInput {
            field: "id".to_string(),
            reason: "ID cannot be empty".to_string(),
        });
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn execute(
    project: String,
    main_repo_path: PathBuf,
    level: Level,
    id: String,
    name: String,
    description: Option<String>,
    remote_url: Option<String>,
    api_url: Option<String>,
) -> TmResult<()> {
    // Step 1: Validate inputs
    validate_inputs(&id, &name)?;

    // Step 2: Validate main repo path is a git repository
    git::validate_worktree(&main_repo_path)?;

    // Step 3: Compute worktree path
    let level_str = level.as_str();
    let worktree_path = git::compute_worktree_path(&main_repo_path, level_str, &id, &name)?;

    // Step 4: Check if worktree already exists
    if worktree_path.exists() {
        return Err(TmError::WorktreeAlreadyExists {
            path: worktree_path,
        });
    }

    // Step 5: Ensure parent directory exists
    if let Some(parent) = worktree_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Step 6: Generate branch name
    let branch_name = git::generate_branch_name(level_str, &id, &name);

    // Step 7: Create worktree (always, this is now default behavior)
    git::create_worktree(&main_repo_path, &worktree_path, &branch_name, None)?;

    println!("Created worktree at: {}", worktree_path.display());
    println!("Branch: {}", branch_name);

    // Step 8: Generate task title from components
    // Format: "{level}/{id}-{name_kebab_case}" for uniqueness and consistency with branch name
    let name_kebab = git::to_kebab_case(&name);
    let task_title = format!("{}/{}-{}", level_str, id, name_kebab);

    // Step 9: Create task with builder pattern
    let mut task = Task::new(task_title.clone(), worktree_path.clone()).with_reference(id.clone());

    if let Some(desc) = description {
        task = task.with_description(desc);
    }
    if let Some(url) = remote_url {
        task = task.with_remote_url(url);
    }
    if let Some(url) = api_url {
        task = task.with_api_url(url);
    }

    // Step 10: Load storage and add task
    let tasks_file = get_tasks_file_path()?;
    let mut storage = TaskStorage::load(&tasks_file)?;
    storage.add_task(project.clone(), task)?;
    storage.save(&tasks_file)?;

    println!("Added task '{}' to project '{}'", task_title, project);

    Ok(())
}

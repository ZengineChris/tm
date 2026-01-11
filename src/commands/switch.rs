use crate::config::get_tasks_file_path;
use crate::error::TmResult;
use crate::git::validate_worktree;
use crate::models::storage::TaskStorage;

pub fn execute(project: String, title: String) -> TmResult<()> {
    let tasks_file = get_tasks_file_path()?;
    let storage = TaskStorage::load(&tasks_file)?;

    let task = storage.get_task(&project, &title)?;

    // Validate worktree still exists
    validate_worktree(&task.worktree_path)?;

    // Output only the path for shell integration
    println!("{}", task.worktree_path.display());

    Ok(())
}

use crate::config::get_tasks_file_path;
use crate::error::TmResult;
use crate::git::remove_worktree;
use crate::models::storage::TaskStorage;

pub fn execute(
    project: String,
    title: String,
    remove_worktree_flag: bool,
    force: bool,
) -> TmResult<()> {
    let tasks_file = get_tasks_file_path()?;
    let mut storage = TaskStorage::load(&tasks_file)?;

    // Get the task before removing it (to get worktree path)
    let task = storage.get_task(&project, &title)?.clone();

    // Remove from storage
    storage.remove_task(&project, &title)?;

    // Save storage
    storage.save(&tasks_file)?;

    println!("Removed task '{}' from project '{}'", title, project);

    // Optionally remove worktree
    if remove_worktree_flag {
        remove_worktree(&task.worktree_path, force)?;
        println!("Removed worktree at: {}", task.worktree_path.display());
    }

    Ok(())
}

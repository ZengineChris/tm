use crate::config::get_tasks_file_path;
use crate::error::TmResult;
use crate::git;
use crate::models::{storage::TaskStorage, task::Task};
use std::path::PathBuf;

pub fn execute(
    project: String,
    title: String,
    worktree_path: PathBuf,
    description: Option<String>,
    reference: Option<String>,
    remote_url: Option<String>,
    api_url: Option<String>,
    create_worktree: bool,
    base_branch: Option<String>,
    main_repo: Option<PathBuf>,
) -> TmResult<()> {
    // If create_worktree is set, create the worktree
    if create_worktree {
        let main_repo_path = main_repo.as_ref().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Main repository path is required when creating worktree",
            )
        })?;

        // Generate branch name from project and title
        let branch_name = format!("{}-{}", project, title)
            .replace(' ', "-")
            .to_lowercase();

        git::create_worktree(
            main_repo_path,
            &worktree_path,
            &branch_name,
            base_branch.as_deref(),
        )?;

        println!("Created worktree at: {}", worktree_path.display());
    } else {
        // Validate that the worktree exists
        git::validate_worktree(&worktree_path)?;
    }

    // Create task
    let mut task = Task::new(title.clone(), worktree_path.clone());

    if let Some(desc) = description {
        task = task.with_description(desc);
    }
    if let Some(ref_id) = reference {
        task = task.with_reference(ref_id);
    }
    if let Some(url) = remote_url {
        task = task.with_remote_url(url);
    }
    if let Some(url) = api_url {
        task = task.with_api_url(url);
    }

    // Load storage
    let tasks_file = get_tasks_file_path()?;
    let mut storage = TaskStorage::load(&tasks_file)?;

    // Add task
    storage.add_task(project.clone(), task)?;

    // Save storage
    storage.save(&tasks_file)?;

    println!("Added task '{}' to project '{}'", title, project);

    Ok(())
}

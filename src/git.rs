use crate::error::{TmError, TmResult};
use git2::Repository;
use std::path::{Path, PathBuf};

/// Validate that a path is a valid git worktree
pub fn validate_worktree(path: &Path) -> TmResult<()> {
    // Check if path exists
    if !path.exists() {
        return Err(TmError::WorktreePathNotFound {
            path: path.to_path_buf(),
        });
    }

    // Try to open as a repository (worktrees are also valid repos)
    match Repository::open(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(TmError::InvalidWorktree {
            path: path.to_path_buf(),
        }),
    }
}

/// Check if worktree has uncommitted changes
pub fn has_uncommitted_changes(path: &Path) -> TmResult<bool> {
    let repo = Repository::open(path)?;

    // Check for staged changes
    let statuses = repo.statuses(None)?;

    Ok(!statuses.is_empty())
}

/// Create a new git worktree
pub fn create_worktree(
    main_repo_path: &Path,
    worktree_path: &Path,
    branch_name: &str,
    base_branch: Option<&str>,
) -> TmResult<()> {
    // Open the main repository
    let repo = Repository::open(main_repo_path).map_err(|_| TmError::GitRepoNotFound {
        path: main_repo_path.to_path_buf(),
    })?;

    // Determine the base commit
    let base_commit = if let Some(base) = base_branch {
        // Find the base branch
        let base_ref = repo.find_reference(&format!("refs/heads/{}", base))?;
        base_ref.peel_to_commit()?
    } else {
        // Use HEAD
        repo.head()?.peel_to_commit()?
    };

    // Create a new branch
    let branch = repo
        .branch(branch_name, &base_commit, false)
        .map_err(|e| TmError::WorktreeCreationFailed {
            path: worktree_path.to_path_buf(),
            reason: format!("Failed to create branch: {}", e),
        })?;

    // Create the worktree
    repo.worktree(
        branch_name,
        worktree_path,
        Some(git2::WorktreeAddOptions::new().reference(Some(branch.get()))),
    )
    .map_err(|e| TmError::WorktreeCreationFailed {
        path: worktree_path.to_path_buf(),
        reason: e.to_string(),
    })?;

    Ok(())
}

/// Remove a git worktree
pub fn remove_worktree(path: &Path, force: bool) -> TmResult<()> {
    // First check if it has uncommitted changes
    if !force && has_uncommitted_changes(path)? {
        return Err(TmError::WorktreeHasChanges {
            path: path.to_path_buf(),
        });
    }

    // Open the worktree repository
    let repo = Repository::open(path)?;

    // Get the main repository path from the worktree and convert to owned PathBuf
    let main_repo_path = repo
        .path()
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .ok_or_else(|| TmError::WorktreeRemovalFailed {
            path: path.to_path_buf(),
            reason: "Could not determine main repository path".to_string(),
        })?;

    // Close the repo before removing the directory
    drop(repo);

    // Remove the actual directory first
    std::fs::remove_dir_all(path).map_err(|e| TmError::WorktreeRemovalFailed {
        path: path.to_path_buf(),
        reason: format!("Failed to remove directory: {}", e),
    })?;

    // Open the main repository and prune worktrees
    let main_repo = Repository::open(main_repo_path).map_err(|e| TmError::WorktreeRemovalFailed {
        path: path.to_path_buf(),
        reason: format!("Failed to open main repository: {}", e),
    })?;

    // Prune invalid worktrees (this will clean up the removed worktree)
    let worktrees = main_repo.worktrees().map_err(|e| TmError::WorktreeRemovalFailed {
        path: path.to_path_buf(),
        reason: format!("Failed to list worktrees: {}", e),
    })?;

    for wt_name in worktrees.iter().flatten() {
        if let Ok(wt) = main_repo.find_worktree(wt_name) {
            let mut prune_options = git2::WorktreePruneOptions::new();
            prune_options.valid(true);
            let _ = wt.prune(Some(&mut prune_options));
        }
    }

    Ok(())
}

/// Get information about a worktree
pub fn get_worktree_info(path: &Path) -> TmResult<WorktreeInfo> {
    let repo = Repository::open(path)?;

    let head = repo.head()?;
    let branch_name = head.shorthand().map(String::from);

    let has_changes = has_uncommitted_changes(path)?;

    Ok(WorktreeInfo {
        path: path.to_path_buf(),
        branch: branch_name,
        has_uncommitted_changes: has_changes,
    })
}

#[derive(Debug)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: Option<String>,
    pub has_uncommitted_changes: bool,
}

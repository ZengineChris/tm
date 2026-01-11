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

    // Get the worktree name (last component of path, used for .git/worktrees/<name>)
    // This must not contain slashes, so we use the directory name
    let worktree_name = worktree_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| TmError::WorktreeCreationFailed {
            path: worktree_path.to_path_buf(),
            reason: "Invalid worktree path".to_string(),
        })?;

    // Create the worktree
    repo.worktree(
        worktree_name,
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: Option<String>,
    pub has_uncommitted_changes: bool,
}

/// Convert a name to snake_case (for filesystem paths)
/// Examples: "Auth System" -> "auth_system", "API-Gateway" -> "api_gateway"
pub fn to_snake_case(s: &str) -> String {
    s.to_lowercase()
        .replace([' ', '-'], "_")
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

/// Convert a name to kebab-case (for git branch names)
/// Examples: "Auth System" -> "auth-system", "API_Gateway" -> "api-gateway"
pub fn to_kebab_case(s: &str) -> String {
    s.to_lowercase()
        .replace([' ', '_'], "-")
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Extract repository root from main branch path
/// Example: "/home/user/projects/myapp/main" -> "/home/user/projects/myapp"
pub fn get_repo_root(main_repo_path: &Path) -> TmResult<PathBuf> {
    let parent = main_repo_path
        .parent()
        .ok_or_else(|| TmError::InvalidMainRepoPath {
            path: main_repo_path.to_path_buf(),
        })?;

    if !parent.exists() {
        return Err(TmError::InvalidMainRepoPath {
            path: main_repo_path.to_path_buf(),
        });
    }

    Ok(parent.to_path_buf())
}

/// Compute worktree path from components
/// Format: <repo_root>/<level>/<id>-<name_snake_case>
pub fn compute_worktree_path(
    main_repo_path: &Path,
    level: &str,
    id: &str,
    name: &str,
) -> TmResult<PathBuf> {
    let repo_root = get_repo_root(main_repo_path)?;
    let name_snake = to_snake_case(name);

    Ok(repo_root.join(level).join(format!("{}-{}", id, name_snake)))
}

/// Generate branch name from components
/// Format: <level>/<id>-<name_kebab_case>
pub fn generate_branch_name(level: &str, id: &str, name: &str) -> String {
    let name_kebab = to_kebab_case(name);
    format!("{}/{}-{}", level, id, name_kebab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Auth System"), "auth_system");
        assert_eq!(to_snake_case("API-Gateway"), "api_gateway");
        assert_eq!(to_snake_case("simple"), "simple");
        assert_eq!(to_snake_case("Multi  Space"), "multi_space");
        assert_eq!(to_snake_case("Mixed-Case_Input"), "mixed_case_input");
        assert_eq!(to_snake_case("UPPERCASE"), "uppercase");
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("Auth System"), "auth-system");
        assert_eq!(to_kebab_case("API_Gateway"), "api-gateway");
        assert_eq!(to_kebab_case("simple"), "simple");
        assert_eq!(to_kebab_case("Multi  Space"), "multi-space");
        assert_eq!(to_kebab_case("Mixed-Case_Input"), "mixed-case-input");
        assert_eq!(to_kebab_case("UPPERCASE"), "uppercase");
        assert_eq!(to_kebab_case("nem plonn"), "nem-plonn");
    }

    #[test]
    fn test_generate_branch_name() {
        assert_eq!(
            generate_branch_name("feature", "JIRA-123", "auth"),
            "feature/JIRA-123-auth"
        );
        assert_eq!(
            generate_branch_name("fix", "BUG-456", "database"),
            "fix/BUG-456-database"
        );
        assert_eq!(
            generate_branch_name("chore", "TASK-1", "cleanup"),
            "chore/TASK-1-cleanup"
        );
        assert_eq!(
            generate_branch_name("feature", "83772", "nem plonn"),
            "feature/83772-nem-plonn"
        );
    }
}

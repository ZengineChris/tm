# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TM (Task Manager) is a CLI tool written in Rust that helps manage tasks in combination with git worktrees. Following the Unix philosophy of "do one thing and do it well," TM provides task management that can integrate with other tools like sesh (tmux session handling) and pm (project management).

Each task is associated with a git worktree and includes:
- Title (format: `<level>/<id>-<name>`, unique within a project)
- Optional description
- Worktree path (automatically computed as `<repo>/<level>/<id>-<name_snake_case>`)
- Reference ID (e.g., JIRA-123)
- Task level (feature, fix, chore, docs, refactor, test, perf, style, ci)
- Optional remote and API URLs (for future GitHub/Jira integration)

Worktree paths follow a structured convention:
- Main repo: `<repo-name>/<main-branch>` (e.g., `~/projects/myapp/main`)
- Worktrees: `<repo-name>/<level>/<id>-<name_snake_case>` (e.g., `~/projects/myapp/feature/JIRA-123-auth`, `~/projects/myapp/feature/83772-nem_plonn`)
- Branch names: `<level>/<id>-<name_kebab_case>` (e.g., `feature/JIRA-123-auth`, `feature/83772-nem-plonn`)
- Task titles: `<level>/<id>-<name_kebab_case>` (matches branch name)

Name conversion:
- **snake_case** (for filesystem paths): spaces and hyphens → underscores (`"nem plonn"` → `nem_plonn`)
- **kebab-case** (for git branches): spaces and underscores → hyphens (`"nem plonn"` → `nem-plonn`)

Tasks are stored in `~/.config/tm/tasks.toml` and grouped by project.

## Development Setup

### Using Nix (Recommended)
```bash
# Enter development shell with all dependencies
nix develop

# Build with Nix
nix build

# Run directly with Nix
nix run
```

### Using Cargo
```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .

# Development mode
cargo run -- <command>
```

### Dependencies
- libgit2, openssl, pkg-config (for git2-rs)
- Rust toolchain (2021 edition)

## Architecture

### Module Structure
```
src/
├── main.rs           - Entry point, CLI parsing, command dispatch
├── cli.rs            - Clap command definitions
├── commands/         - Command implementations
│   ├── add.rs        - Add task and create worktree with structured paths
│   ├── list.rs       - List tasks with filtering
│   ├── remove.rs     - Remove task, optionally remove worktree
│   └── switch.rs     - Output worktree path for shell integration
├── models/
│   ├── task.rs       - Task struct with builder pattern
│   └── storage.rs    - TaskStorage with TOML persistence
├── git.rs            - Git worktree operations (create, remove, validate)
├── error.rs          - TmError enum with user-friendly messages
└── config.rs         - Configuration path helpers
```

### Key Design Patterns

**Error Handling**: Uses thiserror for structured errors with user-friendly messages via `user_message()` method.

**Storage**: HashMap-based storage with TOML serialization. Tasks are uniquely identified by project + title.

**Git Integration**: Uses git2-rs for worktree operations. Automatically creates worktrees with structured paths based on task level, ID, and name.

**CLI Design**: Unix-friendly with minimal output. The `switch` command outputs only the path for shell integration (e.g., `cd $(tm switch project task)`).

## Common Commands

### Build and Test
```bash
cargo build --release        # Build release binary
cargo test                   # Run tests
cargo clippy                 # Run linter
cargo fmt                    # Format code
```

### Usage Examples
```bash
# Add task (creates worktree automatically)
tm add myproject ~/projects/myapp/main --level feature --id JIRA-123 --name auth
# Creates worktree at: ~/projects/myapp/feature/JIRA-123-auth
# Branch: feature/JIRA-123-auth

# Add task with description (short flags)
tm add myproject ~/projects/myapp/main -l fix -i BUG-456 -n database-leak -d "Fix memory leak in database connection pool"

# Add task with all supported levels
tm add myproject ~/projects/myapp/main -l feature -i TASK-1 -n new-feature
tm add myproject ~/projects/myapp/main -l fix -i BUG-2 -n bugfix
tm add myproject ~/projects/myapp/main -l chore -i TASK-3 -n cleanup
tm add myproject ~/projects/myapp/main -l docs -i TASK-4 -n documentation
tm add myproject ~/projects/myapp/main -l refactor -i TASK-5 -n refactor
tm add myproject ~/projects/myapp/main -l test -i TASK-6 -n tests
tm add myproject ~/projects/myapp/main -l perf -i TASK-7 -n performance
tm add myproject ~/projects/myapp/main -l style -i TASK-8 -n styling
tm add myproject ~/projects/myapp/main -l ci -i TASK-9 -n ci-pipeline

# List all tasks
tm list

# List tasks for specific project
tm list -p myproject

# List in JSON format
tm list --format json

# Switch to task (shell integration)
cd $(tm switch myproject feature/JIRA-123-auth)

# Remove task (keep worktree)
tm remove myproject feature/JIRA-123-auth

# Remove task and worktree
tm remove myproject feature/JIRA-123-auth -w

# Force remove even with uncommitted changes
tm remove myproject feature/JIRA-123-auth -w -f
```

## Storage Format

Tasks are stored in `~/.config/tm/tasks.toml`:
```toml
[projects]
myapp = [
    { title = "feature/JIRA-123-auth", worktree_path = "/home/user/projects/myapp/feature/JIRA-123-auth", reference = "JIRA-123", description = "..." }
]
```

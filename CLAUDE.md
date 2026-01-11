# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TM (Task Manager) is a CLI tool written in Rust that helps manage tasks in combination with git worktrees. Following the Unix philosophy of "do one thing and do it well," TM provides task management that can integrate with other tools like sesh (tmux session handling) and pm (project management).

Each task is associated with a git worktree and includes:
- Title (unique within a project)
- Optional description
- Worktree path
- Reference ID (for git commit messages, e.g., JIRA-123)
- Optional remote and API URLs (for future GitHub/Jira integration)

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
│   ├── add.rs        - Add task, optionally create worktree
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

**Git Integration**: Uses git2-rs for worktree operations. Validates worktree paths before storing. Supports both automated and manual worktree management.

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
# Add task with existing worktree
tm add myproject feature-auth -p ~/projects/myapp-auth -r JIRA-123

# Add task and create worktree
tm add myproject feature-db -p ~/projects/myapp-db -c -m ~/projects/myapp -b main

# List all tasks
tm list

# List tasks for specific project
tm list -p myproject

# List in JSON format
tm list --format json

# Switch to task (shell integration)
cd $(tm switch myproject feature-auth)

# Remove task (keep worktree)
tm remove myproject feature-auth

# Remove task and worktree
tm remove myproject feature-auth -w

# Force remove even with uncommitted changes
tm remove myproject feature-auth -w -f
```

## Storage Format

Tasks are stored in `~/.config/tm/tasks.toml`:
```toml
[projects]
myapp = [
    { title = "feature-auth", worktree_path = "/path/to/worktree", reference = "JIRA-123", description = "..." }
]
```

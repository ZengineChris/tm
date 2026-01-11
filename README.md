# TM - Task Manager

A CLI tool for managing tasks with git worktrees, following the Unix philosophy.

## Overview

TM (Task Manager) helps you manage development tasks where each task is associated with a git worktree. It's designed to integrate seamlessly with other tools like `sesh` (tmux session management) and `pm` (project management).

### Features

- Manage tasks with git worktrees
- Automatic or manual worktree creation/removal
- Task metadata: description, reference IDs (JIRA, GitHub issues)
- Shell integration for quick navigation
- Multiple output formats (table, simple, JSON)
- TOML-based storage in `~/.config/tm/tasks.toml`

## Installation

### Using Nix

```bash
nix build
```

### Using Cargo

```bash
cargo install --path .
```

## Quick Start

### Add a task with an existing worktree

```bash
tm add myproject feature-auth \
  --worktree-path ~/projects/myapp-auth \
  --reference JIRA-123 \
  --description "Implement OAuth authentication"
```

### Add a task and create a new worktree

```bash
tm add myproject feature-db \
  --worktree-path ~/projects/myapp-db \
  --create-worktree \
  --main-repo ~/projects/myapp \
  --base-branch main
```

### List tasks

```bash
# List all tasks
tm list

# List tasks for a specific project
tm list --project myproject

# Output as JSON
tm list --format json
```

### Switch to a task

```bash
# For shell integration
cd $(tm switch myproject feature-auth)
```

### Remove a task

```bash
# Remove task from TM (keep worktree)
tm remove myproject feature-auth

# Remove task and delete worktree
tm remove myproject feature-auth --remove-worktree

# Force remove even with uncommitted changes
tm remove myproject feature-auth --remove-worktree --force
```

## Commands

### `tm add`

Add a new task to a project.

**Arguments:**
- `<project>` - Project name
- `<title>` - Task title (unique within project)

**Options:**
- `-p, --worktree-path <PATH>` - Path to the worktree (required)
- `-d, --description <TEXT>` - Task description
- `-r, --reference <ID>` - Reference ID (e.g., JIRA-123)
- `--remote-url <URL>` - Remote URL for future integration
- `--api-url <URL>` - API URL for future integration
- `-c, --create-worktree` - Create the git worktree
- `-b, --base-branch <BRANCH>` - Base branch for worktree (requires `-c`)
- `-m, --main-repo <PATH>` - Main repository path (requires `-c`)

### `tm list`

List tasks with optional filtering.

**Options:**
- `-p, --project <NAME>` - Filter by project name
- `-f, --format <FORMAT>` - Output format: `table` (default), `simple`, or `json`

### `tm remove`

Remove a task from TM.

**Arguments:**
- `<project>` - Project name
- `<title>` - Task title

**Options:**
- `-w, --remove-worktree` - Also remove the git worktree
- `-f, --force` - Force removal even with uncommitted changes (requires `-w`)

### `tm switch`

Output the worktree path for shell integration.

**Arguments:**
- `<project>` - Project name
- `<title>` - Task title

**Usage:**
```bash
cd $(tm switch myproject feature-auth)
```

## Storage

Tasks are stored in `~/.config/tm/tasks.toml`:

```toml
[projects]
myapp = [
    {
      title = "feature-auth",
      worktree_path = "/home/user/projects/myapp-auth",
      reference = "JIRA-123",
      description = "Implement OAuth authentication"
    }
]
```

## Development

### With Nix

```bash
# Enter development shell
nix develop

# Build
nix build

# Run
nix run -- list
```

### With Cargo

```bash
# Build
cargo build

# Run tests
cargo test

# Run with arguments
cargo run -- add myproject task1 -p /tmp/worktree

# Format code
cargo fmt

# Lint
cargo clippy
```

## Integration Examples

### Shell Function

Add to your `.bashrc` or `.zshrc`:

```bash
# Quick switch to a task
tms() {
  cd $(tm switch "$1" "$2")
}

# Usage: tms myproject feature-auth
```

### With tmux/sesh

```bash
# Create a new tmux session for a task
tm-session() {
  local project=$1
  local task=$2
  local path=$(tm switch "$project" "$task")
  tmux new-session -s "${project}-${task}" -c "$path"
}
```

## License

MIT

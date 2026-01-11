use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod commands;
mod config;
mod error;
mod git;
mod models;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add {
            project,
            title,
            worktree_path,
            description,
            reference,
            remote_url,
            api_url,
            create_worktree,
            base_branch,
            main_repo,
        } => commands::add::execute(
            project,
            title,
            worktree_path,
            description,
            reference,
            remote_url,
            api_url,
            create_worktree,
            base_branch,
            main_repo,
        ),
        Commands::List { project, format } => commands::list::execute(project, format),
        Commands::Remove {
            project,
            title,
            remove_worktree,
            force,
        } => commands::remove::execute(project, title, remove_worktree, force),
        Commands::Switch { project, title } => commands::switch::execute(project, title),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e.user_message());
        std::process::exit(1);
    }
}

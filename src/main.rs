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
            main_repo_path,
            level,
            id,
            name,
            description,
            remote_url,
            api_url,
        } => commands::add::execute(
            project,
            main_repo_path,
            level,
            id,
            name,
            description,
            remote_url,
            api_url,
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

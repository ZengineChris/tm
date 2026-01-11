use crate::cli::OutputFormat;
use crate::config::get_tasks_file_path;
use crate::error::TmResult;
use crate::models::storage::TaskStorage;
use colored::Colorize;

pub fn execute(project: Option<String>, format: OutputFormat) -> TmResult<()> {
    let tasks_file = get_tasks_file_path()?;
    let storage = TaskStorage::load(&tasks_file)?;

    let tasks = storage.list_tasks(project.as_deref());

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    match format {
        OutputFormat::Table => print_table(&tasks),
        OutputFormat::Simple => print_simple(&tasks),
        OutputFormat::Json => print_json(&tasks)?,
    }

    Ok(())
}

fn print_table(tasks: &[(&str, &crate::models::task::Task)]) {
    // Header
    println!(
        "{:<20} {:<30} {:<50} {:<15}",
        "PROJECT".bold(),
        "TITLE".bold(),
        "WORKTREE PATH".bold(),
        "REFERENCE".bold()
    );
    println!("{}", "-".repeat(120));

    // Tasks
    for (project, task) in tasks {
        println!(
            "{:<20} {:<30} {:<50} {:<15}",
            project,
            task.title,
            task.worktree_path.display(),
            task.reference.as_deref().unwrap_or("-")
        );
    }
}

fn print_simple(tasks: &[(&str, &crate::models::task::Task)]) {
    for (project, task) in tasks {
        println!("{}/{}", project, task.title);
    }
}

fn print_json(tasks: &[(&str, &crate::models::task::Task)]) -> TmResult<()> {
    use serde_json::json;

    let json_tasks: Vec<_> = tasks
        .iter()
        .map(|(project, task)| {
            json!({
                "project": project,
                "title": task.title,
                "worktree_path": task.worktree_path,
                "description": task.description,
                "reference": task.reference,
                "remote_url": task.remote_url,
                "api_url": task.api_url,
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&json_tasks)?);

    Ok(())
}

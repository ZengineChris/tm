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
    // Calculate column widths based on content
    let mut max_project = "PROJECT".len();
    let mut max_title = "TITLE".len();
    let mut max_reference = "REFERENCE".len();

    for (project, task) in tasks {
        max_project = max_project.max(project.len());
        max_title = max_title.max(task.title.len());
        if let Some(ref r) = task.reference {
            max_reference = max_reference.max(r.len());
        }
    }

    // Add some padding
    max_project += 2;
    max_title += 2;
    max_reference += 2;

    // Header
    println!(
        "{:<project_w$}{:<title_w$}{:<ref_w$}{}",
        "PROJECT".bold(),
        "TITLE".bold(),
        "REFERENCE".bold(),
        "WORKTREE PATH".bold(),
        project_w = max_project,
        title_w = max_title,
        ref_w = max_reference,
    );

    // Tasks
    for (project, task) in tasks {
        println!(
            "{:<project_w$}{:<title_w$}{:<ref_w$}{}",
            project,
            task.title,
            task.reference.as_deref().unwrap_or("-"),
            task.worktree_path.display(),
            project_w = max_project,
            title_w = max_title,
            ref_w = max_reference,
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

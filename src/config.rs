use crate::error::TmResult;
use std::path::PathBuf;

/// Get the path to the tasks.toml file
pub fn get_tasks_file_path() -> TmResult<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine config directory",
        )
    })?;

    Ok(config_dir.join("tm").join("tasks.toml"))
}

/// Ensure the config directory exists
#[allow(dead_code)]
pub fn ensure_config_dir() -> TmResult<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine config directory",
            )
        })?
        .join("tm");

    std::fs::create_dir_all(&config_dir)?;

    Ok(config_dir)
}

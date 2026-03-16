use std::fs;
use std::io;
use std::path::PathBuf;

pub fn global_version_path() -> io::Result<PathBuf> {
    let home = home_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "unable to resolve home directory",
        )
    })?;
    Ok(home.join(".mentalogue").join("stratum").join("global"))
}

pub fn get_global_version() -> io::Result<Option<String>> {
    let path = global_version_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        return Ok(Some(trimmed.to_string()));
    }
    Ok(None)
}

pub fn set_global_version(version: &str) -> io::Result<()> {
    let path = global_version_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{version}\n"))
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(value) = std::env::var("HOME") {
        if !value.is_empty() {
            return Some(PathBuf::from(value));
        }
    }
    if let Ok(value) = std::env::var("USERPROFILE") {
        if !value.is_empty() {
            return Some(PathBuf::from(value));
        }
    }
    None
}

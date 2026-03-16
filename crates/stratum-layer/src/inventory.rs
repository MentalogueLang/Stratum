use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::layer::Layer;

pub fn layers_root() -> io::Result<PathBuf> {
    let home = home_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "unable to resolve home directory",
        )
    })?;
    Ok(home.join(".mentalogue").join("stratum").join("layers"))
}

pub fn list_layers() -> io::Result<Vec<Layer>> {
    let root = layers_root()?;
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut layers = Vec::new();
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(version) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        layers.push(Layer::new(version.to_string(), path));
    }

    layers.sort_by(|left, right| left.version.cmp(&right.version));
    Ok(layers)
}

pub fn find_layer(version: &str) -> io::Result<Option<Layer>> {
    let root = layers_root()?;
    let path = root.join(version);
    if path.is_dir() {
        Ok(Some(Layer::new(version.to_string(), path)))
    } else {
        Ok(None)
    }
}

pub fn layer_path(version: &str) -> io::Result<PathBuf> {
    let root = layers_root()?;
    Ok(root.join(version))
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

pub fn ensure_layers_dir(root: &Path) -> io::Result<()> {
    if !root.exists() {
        fs::create_dir_all(root)?;
    }
    Ok(())
}

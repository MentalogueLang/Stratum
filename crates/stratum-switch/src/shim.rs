use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use stratum_layer::find_layer;
use stratum_pin::resolve_pin;

use crate::global::get_global_version;

pub fn ensure_shim_ready() -> io::Result<()> {
    let bin_dir = shim_bin_dir()?;
    fs::create_dir_all(&bin_dir)?;
    let added = ensure_bin_on_path(&bin_dir)?;
    if added {
        println!("added Stratum shim to PATH; restart your shell to use `inscribe`");
    }
    match ensure_inscribe_shim() {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn ensure_inscribe_shim() -> io::Result<PathBuf> {
    let bin_dir = shim_bin_dir()?;
    fs::create_dir_all(&bin_dir)?;
    let shim_path = bin_dir.join("inscribe.cmd");
    let contents = build_inscribe_shim()?;
    fs::write(&shim_path, contents)?;
    Ok(shim_path)
}

pub fn shim_bin_dir() -> io::Result<PathBuf> {
    let base = std::env::var("LOCALAPPDATA")
        .or_else(|_| std::env::var("APPDATA"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    Ok(base.join("Mentalogue").join("Stratum").join("bin"))
}

fn build_inscribe_shim() -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let local_version = resolve_pin(&cwd)?.map(|pin| pin.version);
    let global_version = get_global_version()?;
    let version = local_version
        .or(global_version)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no active stratum version"))?;

    let layer = find_layer(&version)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("stratum version {version} is not installed"),
        )
    })?;
    let inscribe = layer.bin_dir().join("inscribe.exe");

    let command = format!(
        "@echo off\r\n\"{}\" %*\r\n",
        inscribe.display()
    );
    Ok(command)
}

fn ensure_bin_on_path(bin_dir: &Path) -> io::Result<bool> {
    let current = std::env::var("PATH").unwrap_or_default();
    if path_contains(&current, bin_dir) {
        return Ok(false);
    }

    let updated = if current.trim().is_empty() {
        bin_dir.to_string_lossy().to_string()
    } else if cfg!(windows) {
        format!("{};{}", bin_dir.display(), current)
    } else {
        format!("{}:{}", bin_dir.display(), current)
    };

    std::env::set_var("PATH", &updated);
    if cfg!(windows) {
        let status = Command::new("setx").arg("PATH").arg(&updated).status()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to persist PATH with setx",
            ));
        }
    }

    Ok(true)
}

fn path_contains(path_value: &str, target: &Path) -> bool {
    let delimiter = if cfg!(windows) { ';' } else { ':' };
    let target = normalize_path(&target.to_string_lossy(), cfg!(windows));
    for entry in path_value.split(delimiter) {
        if normalize_path(entry, cfg!(windows)) == target {
            return true;
        }
    }
    false
}

fn normalize_path(path: &str, windows: bool) -> String {
    let trimmed = path.trim().trim_end_matches('\\').trim_end_matches('/');
    if windows {
        trimmed.to_ascii_lowercase()
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_inscribe_path(start: &Path) -> io::Result<PathBuf> {
    let local_version = resolve_pin(start)?.map(|pin| pin.version);
    let global_version = get_global_version()?;
    let version = local_version
        .or(global_version)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no active stratum version"))?;
    let layer = find_layer(&version)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("stratum version {version} is not installed"),
        )
    })?;
    Ok(layer.bin_dir().join("inscribe.exe"))
}

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use stratum_layer::find_layer;
use stratum_pin::resolve_pin;

use crate::global::get_global_version;

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

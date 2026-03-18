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
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            clear_inscribe_shim()?;
            Ok(())
        }
        Err(error) => Err(error),
    }
}

pub fn ensure_inscribe_shim() -> io::Result<PathBuf> {
    let bin_dir = shim_bin_dir()?;
    fs::create_dir_all(&bin_dir)?;
    let shim_path = if cfg!(windows) {
        bin_dir.join("inscribe.cmd")
    } else {
        bin_dir.join("inscribe")
    };
    let contents = build_inscribe_shim()?;
    fs::write(&shim_path, contents)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&shim_path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&shim_path, permissions)?;
    }
    Ok(shim_path)
}

pub fn shim_bin_dir() -> io::Result<PathBuf> {
    if cfg!(windows) {
        let base = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        Ok(base.join("Mentalogue").join("Stratum").join("bin"))
    } else {
        let home = home_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "unable to resolve home directory",
            )
        })?;
        Ok(home.join(".local").join("bin"))
    }
}

fn build_inscribe_shim() -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let local_version = resolve_pin(&cwd)?.map(|pin| pin.version);
    let global_version = get_global_version()?;
    let version = local_version
        .or(global_version)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no active stratum version"))?;
    let inscribe = active_inscribe_path(&version)?;

    if cfg!(windows) {
        Ok(format!("@echo off\r\n\"{}\" %*\r\n", inscribe.display()))
    } else {
        Ok(format!("#!/bin/sh\n\"{}\" \"$@\"\n", inscribe.display()))
    }
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
        persist_path_windows(bin_dir)?;
    } else {
        let profile = user_profile_path().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "unable to resolve shell profile for PATH update",
            )
        })?;
        let export_line = format!("export PATH=\"{}:$PATH\"", bin_dir.display());
        let contents = fs::read_to_string(&profile).unwrap_or_default();
        if !contents.contains(bin_dir.to_string_lossy().as_ref()) {
            let mut updated_profile = contents;
            if !updated_profile.ends_with('\n') && !updated_profile.is_empty() {
                updated_profile.push('\n');
            }
            updated_profile.push_str("# stratum\n");
            updated_profile.push_str(&export_line);
            updated_profile.push('\n');
            fs::write(&profile, updated_profile)?;
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

fn clear_inscribe_shim() -> io::Result<()> {
    let bin_dir = shim_bin_dir()?;
    let shim_path = if cfg!(windows) {
        bin_dir.join("inscribe.cmd")
    } else {
        bin_dir.join("inscribe")
    };
    if shim_path.exists() {
        fs::remove_file(shim_path)?;
    }
    Ok(())
}

fn persist_path_windows(bin_dir: &Path) -> io::Result<()> {
    let machine_script = concat!(
        "$bin = $env:STRATUM_PATH_ENTRY; ",
        "if (-not $bin) { exit 1 }; ",
        "$existing = [Environment]::GetEnvironmentVariable('Path','Machine'); ",
        "if (-not $existing) { $updated = $bin } ",
        "elseif ($existing -notlike \"*$bin*\") { $updated = \"$bin;$existing\" } ",
        "else { exit 0 }; ",
        "try { [Environment]::SetEnvironmentVariable('Path', $updated, 'Machine') } ",
        "catch { exit 1 }"
    );
    let user_script = concat!(
        "$bin = $env:STRATUM_PATH_ENTRY; ",
        "if (-not $bin) { exit 1 }; ",
        "$existing = [Environment]::GetEnvironmentVariable('Path','User'); ",
        "if (-not $existing) { $updated = $bin } ",
        "elseif ($existing -notlike \"*$bin*\") { $updated = \"$bin;$existing\" } ",
        "else { exit 0 }; ",
        "try { [Environment]::SetEnvironmentVariable('Path', $updated, 'User') } ",
        "catch { exit 1 }"
    );
    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(machine_script)
        .env("STRATUM_PATH_ENTRY", bin_dir.to_string_lossy().to_string())
        .status()?;
    if status.success() {
        return Ok(());
    }

    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(user_script)
        .env("STRATUM_PATH_ENTRY", bin_dir.to_string_lossy().to_string())
        .status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to persist PATH via PowerShell",
        ));
    }
    Ok(())
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

fn user_profile_path() -> Option<PathBuf> {
    let home = home_dir()?;
    let profile = home.join(".profile");
    Some(profile)
}

pub fn resolve_inscribe_path(start: &Path) -> io::Result<PathBuf> {
    let local_version = resolve_pin(start)?.map(|pin| pin.version);
    let global_version = get_global_version()?;
    let version = local_version
        .or(global_version)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no active stratum version"))?;
    active_inscribe_path(&version)
}

fn active_inscribe_path(version: &str) -> io::Result<PathBuf> {
    let layer = find_layer(version)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("stratum version {version} is not installed"),
        )
    })?;
    let inscribe = if cfg!(windows) {
        layer.path.join("inscribe-cli.exe")
    } else {
        layer.path.join("inscribe-cli")
    };
    Ok(inscribe)
}

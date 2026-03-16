use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseTarget {
    WindowsX64,
    LinuxX64,
}

impl ReleaseTarget {
    fn suffix(self) -> &'static str {
        match self {
            Self::WindowsX64 => "windows-x64",
            Self::LinuxX64 => "linux-x64",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::WindowsX64 => "zip",
            Self::LinuxX64 => "tar.gz",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReleaseAsset {
    pub version: String,
    pub target: ReleaseTarget,
    pub filename: String,
    pub url: String,
}

pub fn host_target() -> ReleaseTarget {
    if cfg!(target_os = "windows") {
        ReleaseTarget::WindowsX64
    } else {
        ReleaseTarget::LinuxX64
    }
}

pub fn release_asset(version: &str, target: ReleaseTarget) -> ReleaseAsset {
    let version = normalize_version(version);
    let filename = format!(
        "inscribe-{}-{}.{}",
        version,
        target.suffix(),
        target.extension()
    );
    let url = format!(
        "https://github.com/MentalogueLang/Inscribe/releases/download/v{version}/{filename}"
    );
    ReleaseAsset {
        version,
        target,
        filename,
        url,
    }
}

pub fn download_release(version: &str, target: ReleaseTarget) -> io::Result<PathBuf> {
    let asset = release_asset(version, target);
    let response = ureq::get(&asset.url)
        .call()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;
    let mut reader = response.into_reader();
    let path = temp_archive_path(&asset.filename);
    let mut file = File::create(&path)?;
    io::copy(&mut reader, &mut file)?;
    Ok(path)
}

fn normalize_version(version: &str) -> String {
    version.trim().trim_start_matches('v').to_string()
}

fn temp_archive_path(filename: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should move forward")
        .as_nanos();
    std::env::temp_dir().join(format!("stratum_{stamp}_{filename}"))
}

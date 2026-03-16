use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pinfile {
    pub version: String,
}

impl Pinfile {
    pub fn read(path: &Path) -> io::Result<Self> {
        let source = fs::read_to_string(path)?;
        Self::parse(&source).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid pinfile `{}`", path.display()),
            )
        })
    }

    pub fn write(path: &Path, version: &str) -> io::Result<()> {
        let contents = format!("version = \"{version}\"\n");
        fs::write(path, contents)
    }

    pub fn path_for_dir(dir: &Path) -> PathBuf {
        dir.join(".stratum")
    }

    fn parse(source: &str) -> Option<Self> {
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some(version) = parse_version_line(trimmed) {
                return Some(Self { version });
            }
        }
        None
    }
}

fn parse_version_line(line: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix("version") {
        let rest = rest.trim_start();
        if let Some(value) = rest.strip_prefix('=') {
            return Some(trim_quotes(value.trim()));
        }
    }
    Some(trim_quotes(line))
}

fn trim_quotes(value: &str) -> String {
    let mut trimmed = value.trim().to_string();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed = trimmed[1..trimmed.len() - 1].to_string();
    }
    trimmed
}

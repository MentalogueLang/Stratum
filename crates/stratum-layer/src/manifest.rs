use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerManifest {
    pub version: String,
    pub bins: Vec<String>,
}

impl LayerManifest {
    pub fn read(path: &Path) -> io::Result<Self> {
        let source = fs::read_to_string(path)?;
        Self::parse(&source).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid manifest `{}`", path.display()),
            )
        })
    }

    fn parse(source: &str) -> Option<Self> {
        let mut version = None;
        let mut bins = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("version") {
                if let Some(value) = rest.trim_start().strip_prefix('=') {
                    version = Some(trim_quotes(value.trim()));
                }
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("bin") {
                if let Some(value) = rest.trim_start().strip_prefix('=') {
                    bins.push(trim_quotes(value.trim()));
                }
            }
        }
        version.map(|version| LayerManifest { version, bins })
    }
}

fn trim_quotes(value: &str) -> String {
    let mut trimmed = value.trim().to_string();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed = trimmed[1..trimmed.len() - 1].to_string();
    }
    trimmed
}

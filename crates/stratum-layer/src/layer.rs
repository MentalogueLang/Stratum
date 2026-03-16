use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub version: String,
    pub path: PathBuf,
}

impl Layer {
    pub fn new(version: String, path: PathBuf) -> Self {
        Self { version, path }
    }

    pub fn bin_dir(&self) -> PathBuf {
        self.path.join("bin")
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.path.join("layer.toml")
    }

    pub fn exists(&self) -> bool {
        self.path.is_dir()
    }

    pub fn name(&self) -> &str {
        self.version.as_str()
    }

    pub fn path_ref(&self) -> &Path {
        &self.path
    }
}

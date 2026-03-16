use std::io;
use std::path::{Path, PathBuf};

use crate::pinfile::Pinfile;

pub fn find_pinfile(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_file() {
        start.parent().map(Path::to_path_buf)?
    } else {
        start.to_path_buf()
    };

    loop {
        let candidate = Pinfile::path_for_dir(&current);
        if candidate.exists() {
            return Some(candidate);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

pub fn resolve_pin(start: &Path) -> io::Result<Option<Pinfile>> {
    let Some(path) = find_pinfile(start) else {
        return Ok(None);
    };
    Pinfile::read(&path).map(Some)
}

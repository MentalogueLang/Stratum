use std::path::PathBuf;

use stratum_pin::Pinfile;

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 1 {
        return Err("usage: stratum pin <version>".to_string());
    }

    let version = args[0].trim();
    if version.is_empty() {
        return Err("pin version cannot be empty".to_string());
    }

    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    let path = Pinfile::path_for_dir(&cwd);
    Pinfile::write(&path, version).map_err(|error| error.to_string())?;
    println!("pinned {version} in {}", display_path(&path));
    Ok(())
}

fn display_path(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

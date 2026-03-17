use std::fs;

use stratum_layer::find_layer;
use stratum_switch::{get_global_version, resolve_local_version, set_global_version};

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 1 {
        return Err("usage: stratum remove <version>".to_string());
    }

    let version = args[0].trim().trim_start_matches('v');
    if version.is_empty() {
        return Err("remove version cannot be empty".to_string());
    }

    let Some(layer) = find_layer(version).map_err(|error| error.to_string())? else {
        return Err(format!("version {version} is not installed"));
    };

    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    if let Some(active) = resolve_local_version(&cwd).map_err(|error| error.to_string())? {
        if active == version {
            println!("warning: {version} is pinned locally; remove .stratum if needed");
        }
    }

    if let Some(active) = get_global_version().map_err(|error| error.to_string())? {
        if active == version {
            set_global_version("").map_err(|error| error.to_string())?;
            println!("cleared global version {version}");
        }
    }

    fs::remove_dir_all(&layer.path).map_err(|error| error.to_string())?;
    println!("removed {version}");
    Ok(())
}

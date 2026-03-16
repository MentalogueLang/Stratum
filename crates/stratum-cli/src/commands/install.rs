use std::fs;

use stratum_fetch::{download_release, host_target, release_asset, unpack_archive, verify_archive};
use stratum_layer::{ensure_layers_dir, layer_path, layers_root};

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 1 {
        return Err("usage: stratum install <version>".to_string());
    }

    let version = args[0].trim().trim_start_matches('v');
    if version.is_empty() {
        return Err("install version cannot be empty".to_string());
    }

    let root = layers_root().map_err(|error| error.to_string())?;
    ensure_layers_dir(&root).map_err(|error| error.to_string())?;
    let destination = layer_path(version).map_err(|error| error.to_string())?;
    if destination.exists() {
        return Err(format!("version {version} is already installed"));
    }

    let target = host_target();
    let asset = release_asset(version, target);
    println!("downloading {}", asset.url);
    let archive = download_release(version, target).map_err(|error| error.to_string())?;
    verify_archive(&archive).map_err(|error| error.to_string())?;
    fs::create_dir_all(&destination).map_err(|error| error.to_string())?;
    unpack_archive(&archive, &destination).map_err(|error| error.to_string())?;
    let _ = fs::remove_file(&archive);
    println!("installed {version} to {}", destination.display());
    Ok(())
}

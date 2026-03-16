use stratum_layer::find_layer;
use stratum_switch::{get_global_version, resolve_local_version};

pub fn run(_args: &[String]) -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    if let Some(version) = resolve_local_version(&cwd).map_err(|error| error.to_string())? {
        if let Some(layer) = find_layer(&version).map_err(|error| error.to_string())? {
            println!("{}", layer.path.display());
        } else {
            println!("{version}");
        }
        return Ok(());
    }

    if let Some(version) = get_global_version().map_err(|error| error.to_string())? {
        if let Some(layer) = find_layer(&version).map_err(|error| error.to_string())? {
            println!("{}", layer.path.display());
        } else {
            println!("{version}");
        }
        return Ok(());
    }

    Err("no active layer; run `stratum pin <version>` or `stratum use <version>`".to_string())
}

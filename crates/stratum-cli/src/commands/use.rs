use stratum_layer::find_layer;
use stratum_switch::set_global_version;

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 1 {
        return Err("usage: stratum use <version>".to_string());
    }

    let version = args[0].trim().trim_start_matches('v');
    if version.is_empty() {
        return Err("use version cannot be empty".to_string());
    }

    if find_layer(version)
        .map_err(|error| error.to_string())?
        .is_none()
    {
        return Err(format!(
            "version {version} is not installed; run `stratum install {version}`"
        ));
    }

    set_global_version(version).map_err(|error| error.to_string())?;
    println!("using {version}");
    Ok(())
}

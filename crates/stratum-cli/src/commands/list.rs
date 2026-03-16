use stratum_layer::list_layers;

pub fn run(_args: &[String]) -> Result<(), String> {
    let layers = list_layers().map_err(|error| error.to_string())?;
    if layers.is_empty() {
        println!("no layers installed");
        return Ok(());
    }
    for layer in layers {
        println!("{}", layer.version);
    }
    Ok(())
}

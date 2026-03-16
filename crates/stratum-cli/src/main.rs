use stratum_fetch as _;
use stratum_layer as _;
use stratum_pin as _;
use stratum_switch as _;

pub mod commands;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(message) = commands::dispatch(&args) {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

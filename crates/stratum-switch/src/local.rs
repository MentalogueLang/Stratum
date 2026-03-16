use std::io;
use std::path::Path;

use stratum_pin::resolve_pin;

pub fn resolve_local_version(start: &Path) -> io::Result<Option<String>> {
    resolve_pin(start).map(|pin| pin.map(|pin| pin.version))
}

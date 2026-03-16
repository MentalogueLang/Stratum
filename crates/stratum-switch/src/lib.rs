use stratum_layer as _;
use stratum_pin as _;

pub mod shim;
pub mod global;
pub mod local;

pub use global::{get_global_version, set_global_version};
pub use local::resolve_local_version;

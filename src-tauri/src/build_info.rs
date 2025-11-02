use serde::{Deserialize, Serialize};
use serde_json;

pub const BUILD_INFO_JSON : &str = include_str!(concat!(env!("OUT_DIR"), "/build_info.txt"));

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInfo {
    pub timestamp: String,
    pub version: String,
    pub target: String,
}

lazy_static::lazy_static! {
    pub static ref BUILD_INFO: BuildInfo = serde_json::from_str(BUILD_INFO_JSON)
        .expect("Invalid JSON in CONFIG_JSON");
}
use std::{env, io::Write, fs};
use serde::{Deserialize, Serialize};
use serde_json;
use chrono;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildInfo {
    pub timestamp: String,
    pub version: String,
    pub target: String,
}

fn write_build_info_to_file() {
    let build_info = BuildInfo {
        timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        version: env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string()),
        target: env::var("MURMURE_BUILD_TARGET").unwrap_or_else(|_| "unknown".to_string()),
    };
    let outdir = env::var("OUT_DIR").unwrap(); // Official Cargo-set env var
    let outfile = format!("{}/build_info.txt", outdir);

    let mut file_handler = fs::File::create(&outfile).unwrap();
    // Serialize build_info as JSON and write to file
    writeln!(file_handler, "{}", serde_json::to_string_pretty(&build_info).unwrap()).unwrap();
}

fn main() {
    write_build_info_to_file();

    tauri_build::build()
}

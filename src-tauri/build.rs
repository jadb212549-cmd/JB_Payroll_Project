use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:warning==== [RUST BUILD DIAGNOSTIC] START ===");

    // 1. Working Directory
    if let Ok(cwd) = env::current_dir() {
        println!("cargo:warning=[CWD] {}", cwd.display());
    } else {
        println!("cargo:warning=[CWD] Failed to get current working directory");
    }

    // 2. Key Cargo Environment Variables
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "NOT_SET".to_string());
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| "NOT_SET".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "NOT_SET".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "NOT_SET".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "NOT_SET".to_string());

    println!("cargo:warning=[ENV] CARGO_MANIFEST_DIR = {}", manifest_dir);
    println!("cargo:warning=[ENV] OUT_DIR = {}", out_dir);
    println!("cargo:warning=[ENV] TARGET = {}", target);
    println!("cargo:warning=[ENV] HOST = {}", host);
    println!("cargo:warning=[ENV] PROFILE = {}", profile);

    // 3. Path Resolution Validations
    let manifest_path = PathBuf::from(&manifest_dir);

    let tauri_conf_path = manifest_path.join("tauri.conf.json");
    println!(
        "cargo:warning=[PATH CHECK] tauri.conf.json ({}) -> exists: {}",
        tauri_conf_path.display(),
        tauri_conf_path.exists()
    );

    let dist_dir = manifest_path.join("../dist");
    println!(
        "cargo:warning=[PATH CHECK] ../dist ({}) -> exists: {}, is_dir: {}",
        dist_dir.display(),
        dist_dir.exists(),
        dist_dir.is_dir()
    );

    let index_html = dist_dir.join("index.html");
    println!(
        "cargo:warning=[PATH CHECK] ../dist/index.html ({}) -> exists: {}, is_file: {}",
        index_html.display(),
        index_html.exists(),
        index_html.is_file()
    );

    let icons_dir = manifest_path.join("icons");
    println!(
        "cargo:warning=[PATH CHECK] icons dir ({}) -> exists: {}",
        icons_dir.display(),
        icons_dir.exists()
    );

    let icon_files = [
        "32x32.png",
        "128x128.png",
        "128x128@2x.png",
        "icon.icns",
        "icon.ico",
    ];

    for icon in &icon_files {
        let icon_path = icons_dir.join(icon);
        println!(
            "cargo:warning=[PATH CHECK] icon: {} -> exists: {}, size: {:?}",
            icon_path.display(),
            icon_path.exists(),
            fs::metadata(&icon_path).map(|m| m.len()).ok()
        );
    }

    // 4. Dump all environment variables for deep diagnostic inspection
    println!("cargo:warning=--- ALL ENVIRONMENT VARIABLES ---");
    let mut vars: Vec<(String, String)> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));
    for (k, v) in vars {
        // Skip sensitive token strings if any
        if k.contains("KEY") || k.contains("TOKEN") || k.contains("SECRET") {
            println!("cargo:warning=[ENV] {} = [REDACTED]", k);
        } else {
            println!("cargo:warning=[ENV] {} = {}", k, v);
        }
    }
    println!("cargo:warning==== [RUST BUILD DIAGNOSTIC] END ===");

    // Delegate to standard tauri-build
    tauri_build::build()
}

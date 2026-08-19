use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run_diagnostics() {
    println!("cargo:warning==== [RUST BUILD DIAGNOSTICS] START ===");

    // 1. Current Working Directory
    match env::current_dir() {
        Ok(cwd) => println!("cargo:warning=[CWD] {}", cwd.display()),
        Err(e) => println!("cargo:warning=[CWD ERROR] Failed to get CWD: {}", e),
    }

    // 2. Core Cargo & Rust Toolchain Environment Variables
    let cargo_vars = [
        "CARGO_MANIFEST_DIR",
        "OUT_DIR",
        "TARGET",
        "HOST",
        "PROFILE",
        "OPT_LEVEL",
        "DEBUG",
        "RUSTC",
        "RUSTDOC",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_ENV",
        "CARGO_CFG_TARGET_VENDOR",
    ];

    for var_name in &cargo_vars {
        let val = env::var(var_name).unwrap_or_else(|_| "NOT_SET".to_string());
        println!("cargo:warning=[ENV:CARGO] {} = {}", var_name, val);
    }

    // 3. Path Resolution Validations
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let manifest_path = PathBuf::from(&manifest_dir);

    // Check tauri.conf.json
    let tauri_conf_path = manifest_path.join("tauri.conf.json");
    let tauri_conf_exists = tauri_conf_path.exists();
    let tauri_conf_size = fs::metadata(&tauri_conf_path).map(|m| m.len()).unwrap_or(0);
    println!(
        "cargo:warning=[PATH:TAURI_CONF] {} -> exists: {}, size: {} bytes",
        tauri_conf_path.display(),
        tauri_conf_exists,
        tauri_conf_size
    );

    // Check frontend dist folder
    let dist_dir = manifest_path.join("../dist");
    let dist_exists = dist_dir.exists();
    let dist_is_dir = dist_dir.is_dir();
    println!(
        "cargo:warning=[PATH:DIST_DIR] {} -> exists: {}, is_dir: {}",
        dist_dir.display(),
        dist_exists,
        dist_is_dir
    );

    // Check dist/index.html
    let index_html = dist_dir.join("index.html");
    let index_exists = index_html.exists();
    let index_size = fs::metadata(&index_html).map(|m| m.len()).unwrap_or(0);
    println!(
        "cargo:warning=[PATH:INDEX_HTML] {} -> exists: {}, size: {} bytes",
        index_html.display(),
        index_exists,
        index_size
    );

    // Check icon assets
    let icons_dir = manifest_path.join("icons");
    println!(
        "cargo:warning=[PATH:ICONS_DIR] {} -> exists: {}, is_dir: {}",
        icons_dir.display(),
        icons_dir.exists(),
        icons_dir.is_dir()
    );

    let icon_files = [
        "32x32.png",
        "128x128.png",
        "128x128@2x.png",
        "icon.ico",
        "icon.icns",
    ];

    for icon in &icon_files {
        let icon_path = icons_dir.join(icon);
        let exists = icon_path.exists();
        let size = fs::metadata(&icon_path).map(|m| m.len()).unwrap_or(0);
        println!(
            "cargo:warning=[PATH:ICON] {} -> exists: {}, size: {} bytes",
            icon_path.display(),
            exists,
            size
        );
    }

    // 4. System Environment Variables (Redacting Sensitive Tokens)
    println!("cargo:warning=--- SYSTEM ENVIRONMENT VARIABLES ---");
    let mut all_vars: Vec<(String, String)> = env::vars().collect();
    all_vars.sort_by(|a, b| a.0.cmp(&b.0));
    for (k, v) in all_vars {
        if k.contains("KEY") || k.contains("TOKEN") || k.contains("SECRET") || k.contains("AUTH") {
            println!("cargo:warning=[ENV:SYS] {} = [REDACTED]", k);
        } else {
            println!("cargo:warning=[ENV:SYS] {} = {}", k, v);
        }
    }

    println!("cargo:warning==== [RUST BUILD DIAGNOSTICS] END ===");
}

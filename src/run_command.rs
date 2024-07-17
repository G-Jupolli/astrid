use std::path::Path;

use tokio::{fs, process::Command};

pub async fn run_yarn(front_end: bool, dir: String, clear_node: bool) {
    // Build path to directory to run in
    let mut path = match dir.ends_with("/") {
        true => dir,
        false => format!("{}/", dir),
    };

    if front_end {
        path = format!("{}frontend", path);
    }

    println!("path {}", path);

    std::env::set_current_dir(&path).unwrap_or_else(|_| {
        println!(
            "  - \x1b[91mFailed\x1b[0m: Directory {} does not exist",
            path
        );
        std::process::exit(1);
    });

    // Check if there is a package.json file
    if !Path::new("package.json").exists() {
        println!(
            "  - \x1b[91mFailed\x1b[0m: Directory {} does not contain a package.json file",
            path
        );
        std::process::exit(1);
    }

    if clear_node {
        if Path::new("node_modules").exists() {
            match fs::remove_dir_all("node_modules").await {
                Ok(_) => {
                    println!("  - \x1b[92mComplete\x1b[0m: Clearing Node Modules")
                }
                Err(_) => {
                    println!("  - \x1b[91mFailed\x1b[0m: Failed to remove node modules");
                    std::process::exit(1);
                }
            };
        }
    }

    if let Err(e) = Command::new("yarn").arg("install").status().await {
        println!("  - \x1b[91mFailed\x1b[0m: Failed 'yarn install'",);
        println!("{}", e.to_string());
        std::process::exit(1);
    }

    let _ = Command::new("yarn").arg("dev").status().await;
}

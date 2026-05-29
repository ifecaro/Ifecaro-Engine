use std::{fs, path::Path, process::Command, time::SystemTime};

fn main() {
    println!("cargo:rerun-if-changed=.env");

    // Load optional local .env for build-time API/domain overrides used via option_env!.
    dotenv::dotenv().ok();
    for key in [
        "VITE_BASE_API_URL",
        "IFECARO_BASE_API_URL",
        "VITE_STAGING_API_URL",
        "STAGING_API_URL",
        "VITE_PRODUCTION_API_URL",
        "PRODUCTION_API_URL",
        "VITE_APP_ENV",
        "IFECARO_APP_ENV",
        "IFECARO_APP_VERSION",
        "GHCR_TAG",
    ] {
        if let Ok(value) = std::env::var(key) {
            println!("cargo:rustc-env={key}={value}");
        }
    }

    // Allow skipping Tailwind compilation via environment variable
    if std::env::var("SKIP_TAILWIND").is_ok() {
        println!("cargo:warning=Tailwind CSS compilation skipped (SKIP_TAILWIND set)");
        return;
    }

    // Tell Cargo to re-run build script when Tailwind inputs or class content change.
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=tailwind.config.js");

    let tailwind_sources = [Path::new("./src"), Path::new("./tailwind.config.js")];
    let output_path = Path::new("./public/tailwind.css");
    let should_compile = should_compile_tailwind(&tailwind_sources, output_path);

    if should_compile {
        // Make sure Tailwind is available before attempting to compile.
        let tailwind_available = Command::new("tailwindcss")
            .arg("--version")
            .status()
            .map(|status| status.success())
            .unwrap_or(false);

        if !tailwind_available {
            println!(
                "cargo:warning=Tailwind CSS binary not found. Skipping compilation (set SKIP_TAILWIND=1 to silence this)."
            );
            return;
        }

        // Compile Tailwind CSS
        println!("cargo:warning=Starting Tailwind CSS compilation...");
        let tailwind_status = Command::new("tailwindcss")
            .args(["-m", "-i", "./src/input.css", "-o", "./public/tailwind.css"])
            .status()
            .expect("Failed to execute tailwindcss command");

        if !tailwind_status.success() {
            panic!("Tailwind CSS compilation failed");
        }

        println!("cargo:warning=Tailwind CSS compilation complete!");
    } else {
        println!("cargo:warning=Tailwind CSS is up to date, skipping compilation");
    }
}

fn should_compile_tailwind(sources: &[&Path], output_path: &Path) -> bool {
    let output_modified = match modified_time(output_path) {
        Some(modified) => modified,
        None => return true,
    };

    sources
        .iter()
        .filter_map(|source| newest_modified_time(source))
        .any(|source_modified| source_modified > output_modified)
}

fn newest_modified_time(path: &Path) -> Option<SystemTime> {
    let metadata = fs::metadata(path).ok()?;
    let mut newest = metadata.modified().ok();

    if metadata.is_dir() {
        for entry in fs::read_dir(path).ok()?.flatten() {
            if let Some(modified) = newest_modified_time(&entry.path()) {
                newest = Some(newest.map_or(modified, |current| current.max(modified)));
            }
        }
    }

    newest
}

fn modified_time(path: &Path) -> Option<SystemTime> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
}

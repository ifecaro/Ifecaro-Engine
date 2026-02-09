use std::process::Command;

fn main() {
    ensure_wasm_target_installed();

    // Allow skipping Tailwind compilation via environment variable
    if std::env::var("SKIP_TAILWIND").is_ok() {
        println!("cargo:warning=Tailwind CSS compilation skipped (SKIP_TAILWIND set)");
        return;
    }

    // Tell Cargo to re-run build script only when CSS-related files change
    println!("cargo:rerun-if-changed=src/input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    // Only watch specific directories that might contain Tailwind classes
    println!("cargo:rerun-if-changed=src/components/");
    println!("cargo:rerun-if-changed=src/pages/");

    dotenv::dotenv().ok();

    // Check if tailwind.css already exists and is newer than input files
    let tailwind_exists = std::path::Path::new("./public/tailwind.css").exists();
    let should_compile = if tailwind_exists {
        // Check if input.css is newer than output
        let input_modified = std::fs::metadata("./src/input.css")
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        let output_modified = std::fs::metadata("./public/tailwind.css")
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        input_modified > output_modified
    } else {
        true
    };

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

fn ensure_wasm_target_installed() {
    let target = match std::env::var("TARGET") {
        Ok(target) => target,
        Err(_) => return,
    };

    if target != "wasm32-unknown-unknown" {
        return;
    }

    let installed_targets = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();

    let installed_targets = match installed_targets {
        Ok(output) if output.status.success() => output,
        Ok(_) => {
            println!("cargo:warning=Failed to list installed Rust targets.");
            return;
        }
        Err(err) => {
            println!(
                "cargo:warning=Failed to run rustup to list installed targets: {}",
                err
            );
            return;
        }
    };

    let installed_targets = String::from_utf8_lossy(&installed_targets.stdout);
    if installed_targets
        .lines()
        .any(|line| line.trim() == "wasm32-unknown-unknown")
    {
        return;
    }

    println!("cargo:warning=Rust wasm32 target missing. Installing...");
    let install_status = Command::new("rustup")
        .args(["target", "add", "wasm32-unknown-unknown"])
        .status();

    match install_status {
        Ok(status) if status.success() => {
            println!("cargo:warning=Rust wasm32 target installed.");
        }
        Ok(_) => {
            println!("cargo:warning=Failed to install wasm32-unknown-unknown target.");
        }
        Err(err) => {
            println!(
                "cargo:warning=Failed to run rustup to install wasm32 target: {}",
                err
            );
        }
    }
}

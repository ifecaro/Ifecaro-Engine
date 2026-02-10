use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "deploy")]
#[command(about = "Ifecaro Engine Deployment Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Quick check (cargo check)
    Check,
    /// Run test suite (using Rust test runner)
    Test {
        /// Test mode
        #[arg(value_enum)]
        mode: Option<TestMode>,
    },
    /// Build project
    Build,
    /// Staging deployment process (quick check + build + deploy)
    Deploy,
    /// Clean build files
    Clean,
    /// Development mode (build and deploy without tests)
    Dev,
    /// Production mode (optimized full deployment process)
    Prod,
    /// Remote VPS deployment via GHCR (pull image + docker compose up)
    Remote,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TestMode {
    /// Full test suite
    Full,
    /// Quick test
    Quick,
    /// Optimized container test
    Internal,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check) => check()?,
        Some(Commands::Test { mode }) => test(mode.clone())?,
        Some(Commands::Build) => build()?,
        Some(Commands::Deploy) => deploy()?,
        Some(Commands::Clean) => clean()?,
        Some(Commands::Dev) => {
            build()?;
            copy_pwa_resources()?;
            create_deployment_package()?;
            restore_tailwind_css()?;
            upload_to_remote()?;
            println!("{}", "✅ Development deployment completed".green().bold());
        }
        Some(Commands::Prod) => {
            deploy()?;
            println!("{}", "🎉 Production deployment completed".green().bold());
        }
        Some(Commands::Remote) => {
            run_remote_deploy_binary()?;
        }
        None => {
            show_interactive_menu()?;
        }
    }

    Ok(())
}

fn show_interactive_menu() -> Result<()> {
    loop {
        // Simple screen clear impact
        for _ in 0..50 {
            println!();
        }

        println!("{}", "🚀 Ifecaro Engine Deployment Tool".blue().bold());
        println!(
            "{}",
            "================================================".blue()
        );
        println!();

        println!("Please select an operation:");
        println!();
        println!("  {}  📋 Quick check (cargo check)", "1.".cyan().bold());
        println!("  {}  🧪 Run test suite", "2.".cyan().bold());
        println!("  {}  🏗  Build project", "3.".cyan().bold());
        println!("  {}  🧹 Clean build files", "4.".cyan().bold());
        println!(
            "  {}  ⚡ Development mode (build and deploy without tests)",
            "5.".cyan().bold()
        );
        println!(
            "  {}  🎯 Production mode (full one-click deployment)",
            "6.".cyan().bold()
        );
        println!(
            "  {}  🌐 Remote VPS deploy (GHCR pull + docker compose up)",
            "7.".cyan().bold()
        );
        println!("  {}  ❌ Exit", "0.".red().bold());
        println!();

        print!("{}", "Please enter option (0-7): ".green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        println!(); // Empty line

        match choice {
            "1" => {
                println!("{}", "Running quick check...".yellow());
                check()?;
                wait_for_enter();
            }
            "2" => {
                show_test_submenu()?;
            }
            "3" => {
                println!("{}", "Starting project build...".yellow());
                build()?;
                wait_for_enter();
            }
            "4" => {
                println!("{}", "Cleaning build files...".yellow());
                clean()?;
                wait_for_enter();
            }
            "5" => {
                println!("{}", "Starting development mode...".yellow());
                build()?;
                copy_pwa_resources()?;
                create_deployment_package()?;
                restore_tailwind_css()?;
                upload_to_remote()?;
                println!("{}", "✅ Development deployment completed".green().bold());
                wait_for_enter();
            }
            "6" => {
                println!("{}", "Starting production mode...".yellow());
                deploy()?;
                println!("{}", "🎉 Production deployment completed".green().bold());
                wait_for_enter();
            }
            "7" => {
                println!(
                    "{}",
                    "Starting remote VPS deployment (GHCR pull + docker compose up)...".yellow()
                );
                run_remote_deploy_binary()?;
                wait_for_enter();
            }
            "0" => {
                println!(
                    "{}",
                    "Thanks for using Ifecaro Engine Deployment Tool!"
                        .green()
                        .bold()
                );
                break;
            }
            _ => {
                println!(
                    "{}",
                    "Invalid option, please enter again (0-7)".red().bold()
                );
                wait_for_enter();
            }
        }
    }

    Ok(())
}

fn show_test_submenu() -> Result<()> {
    loop {
        // Simple screen clear impact
        for _ in 0..50 {
            println!();
        }

        println!("{}", "🧪 Test Suite Menu".blue().bold());
        println!(
            "{}",
            "================================================".blue()
        );
        println!();

        println!("Please select test mode:");
        println!();
        println!("  {}  🎯 Full test suite (all tests)", "1.".cyan().bold());
        println!(
            "  {}  ⚡ Quick test (compile + basic tests)",
            "2.".cyan().bold()
        );
        println!("  {}  🐳 Optimized container test", "3.".cyan().bold());
        println!("  {}  ↩️ Return to main menu", "0.".yellow().bold());
        println!();

        print!("{}", "Please enter option (0-3): ".green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        println!(); // Empty line

        match choice {
            "1" => {
                println!("{}", "Running full test suite...".yellow());
                test(Some(TestMode::Full))?;
                wait_for_enter();
                break;
            }
            "2" => {
                println!("{}", "Running quick test...".yellow());
                test(Some(TestMode::Quick))?;
                wait_for_enter();
                break;
            }
            "3" => {
                println!("{}", "Running container test...".yellow());
                test(Some(TestMode::Internal))?;
                wait_for_enter();
                break;
            }
            "0" => {
                break; // Return to main menu
            }
            _ => {
                println!(
                    "{}",
                    "Invalid option, please enter again (0-3)".red().bold()
                );
                wait_for_enter();
            }
        }
    }

    Ok(())
}

fn wait_for_enter() {
    println!();
    print!("{}", "Press Enter to continue...".dimmed());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn check() -> Result<()> {
    println!("{}", "🔍 Running Cargo check...".yellow().bold());

    let output = Command::new("cargo")
        .args(&["check", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run cargo check")?;

    if output.success() {
        println!("{}", "✅ Cargo check passed".green().bold());
    } else {
        anyhow::bail!("❌ Cargo check failed");
    }

    Ok(())
}

fn test(mode: Option<TestMode>) -> Result<()> {
    let test_mode = mode.unwrap_or(TestMode::Full);

    println!(
        "{}",
        format!("📋 Running test suite ({:?} mode)...", test_mode)
            .yellow()
            .bold()
    );

    let test_command = match test_mode {
        TestMode::Full => "full",
        TestMode::Quick => "quick",
        TestMode::Internal => "internal",
    };

    // Always use release profile to prevent generating debug artifacts
    let mut cargo_args = vec!["run", "--release"];
    cargo_args.extend_from_slice(&["--bin", "test-runner", test_command]);

    let output = Command::new("cargo")
        .args(&cargo_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run Rust test runner")?;

    if output.success() {
        println!("{}", "✅ Test suite passed".green().bold());
    } else {
        anyhow::bail!("❌ Test suite failed");
    }

    // 新增：自動執行 wasm-pack test
    println!(
        "\n{}",
        "🦀 Running wasm-pack test (headless, Chrome)..."
            .yellow()
            .bold()
    );
    let wasm_pack_result = Command::new("wasm-pack")
        .args(&["test", "--headless", "--chrome", "--release"])
        .env("RUST_LOG", "info")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to run wasm-pack test")?;

    if wasm_pack_result.success() {
        println!("{}", "✅ wasm-pack test passed".green().bold());
    } else {
        anyhow::bail!("❌ wasm-pack test failed");
    }

    Ok(())
}

fn build() -> Result<()> {
    println!("{}", "🏗 Building Rust project...".yellow().bold());

    let rust_build = Command::new("cargo")
        .args(&["build", "--release", "--target", "wasm32-unknown-unknown"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to build Rust project")?;

    if !rust_build.success() {
        anyhow::bail!("❌ Rust build failed");
    }

    println!("{}", "🎯 Building Dioxus project...".yellow().bold());

    let dioxus_build = Command::new("dx")
        .args(&["build", "--release", "--platform", "web"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to build Dioxus project")?;

    if dioxus_build.success() {
        println!("{}", "✅ Build completed".green().bold());
    } else {
        anyhow::bail!("❌ Dioxus build failed");
    }

    Ok(())
}

fn deploy() -> Result<()> {
    println!(
        "{}",
        "🚀 Starting Ifecaro Engine staging deployment process"
            .blue()
            .bold()
    );
    println!(
        "{}",
        "================================================".blue()
    );

    // 1. Run quick check for faster staging iteration
    println!("\n{}", "📋 Running quick cargo check for staging...".yellow().bold());
    let check_result = Command::new("cargo")
        .args(&["check", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run cargo check")?;

    if !check_result.success() {
        anyhow::bail!("❌ Cargo check failed, aborting staging deployment");
    }
    println!("{}", "✅ Cargo check passed".green().bold());

    // 2. Run Rust build
    println!("\n{}", "🏗️ Running Rust build...".yellow().bold());
    let rust_build = Command::new("cargo")
        .args(&["build", "--release", "--target", "wasm32-unknown-unknown"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run Rust build")?;

    if !rust_build.success() {
        anyhow::bail!("❌ Rust build failed");
    }
    println!("{}", "✅ Rust build completed".green().bold());

    // 3. Run Dioxus build
    println!("\n{}", "🎯 Running Dioxus build...".yellow().bold());
    let dioxus_build = Command::new("dx")
        .args(&["build", "--release", "--platform", "web"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run Dioxus build")?;

    if !dioxus_build.success() {
        anyhow::bail!("❌ Dioxus build failed");
    }
    println!("{}", "✅ Dioxus build completed".green().bold());

    // 4. Copy PWA resources
    copy_pwa_resources()?;

    // 5. Create deployment package
    create_deployment_package()?;

    // 6. Restore tailwind.css
    restore_tailwind_css()?;

    // 7. Upload to remote server
    upload_to_remote()?;

    // Optional: clean up debug & incremental artifacts to reduce target size
    cleanup_target_artifacts();

    println!("\n{}", "🎉 Staging deployment process completed!".green().bold());
    println!("Deployment file location: target/dx/ifecaro/release/web/public.tar.gz");

    // Read environment variables for final output
    if let Ok(user) = std::env::var("DEPLOY_USER") {
        if let Ok(host) = std::env::var("DEPLOY_HOST") {
            if let Ok(path) = std::env::var("DEPLOY_PATH") {
                println!("Uploaded to: {}@{}:{}/frontend/", user, host, path);
                return Ok(());
            }
        }
    }
    println!("Uploaded to staging server");

    Ok(())
}

fn copy_pwa_resources() -> Result<()> {
    println!("\n{}", "📦 Copying PWA resources...".yellow().bold());

    let build_dir = "target/dx/ifecaro/release/web";
    let public_dir = format!("{}/public", build_dir);

    // Create directory
    std::fs::create_dir_all(format!("{}/img/icons", public_dir))
        .context("Failed to create directory")?;

    // Copy file function
    let copy_if_exists = |src: &str, dst: &str| {
        if std::path::Path::new(src).exists() {
            if let Err(e) = std::fs::copy(src, dst) {
                println!(
                    "Warning: Failed to copy {} from {} to {}: {}",
                    src, src, dst, e
                );
            }
        } else {
            println!("Warning: {} does not exist", src);
        }
    };

    // Copy root directory files
    copy_if_exists(
        "public/manifest.json",
        &format!("{}/manifest.json", public_dir),
    );
    copy_if_exists("public/sw.js", &format!("{}/sw.js", public_dir));
    copy_if_exists(
        "public/img/icons/favicon.ico",
        &format!("{}/favicon.ico", public_dir),
    );

    // Copy icon files
    if let Ok(entries) = std::fs::read_dir("public/img/icons") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                if let Some(ext) = path.extension() {
                    if ext == "png" || ext == "ico" {
                        let dst =
                            format!("{}/img/icons/{}", public_dir, filename.to_string_lossy());
                        copy_if_exists(&path.to_string_lossy(), &dst);
                    }
                }
            }
        }
    }

    println!("{}", "✅ PWA resources copied".green().bold());
    Ok(())
}

fn create_deployment_package() -> Result<()> {
    println!("\n{}", "📚 Creating deployment package...".yellow().bold());

    let web_dir = "target/dx/ifecaro/release/web";

    let tar_result = Command::new("tar")
        .args(&["-czf", "public.tar.gz", "public/"])
        .current_dir(web_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to create tar package")?;

    if !tar_result.success() {
        anyhow::bail!("❌ Failed to create deployment package");
    }

    println!("{}", "✅ Deployment package created".green().bold());
    Ok(())
}

fn restore_tailwind_css() -> Result<()> {
    println!("\n{}", "🔄 Restoring tailwind.css...".yellow().bold());

    // Ensure git safe directory setting (Docker environment needs)
    let _safe_dir_result = Command::new("git")
        .args(&["config", "--global", "--add", "safe.directory", "/app"])
        .output();

    // First check if file has been modified
    let status_output = Command::new("git")
        .args(&["status", "--porcelain", "public/tailwind.css"])
        .output()
        .context("Failed to check git status")?;

    let status_str = String::from_utf8_lossy(&status_output.stdout);

    if status_str.trim().is_empty() {
        println!(
            "{}",
            "📋 tailwind.css has not been modified, no need to restore".green()
        );
        return Ok(());
    }

    println!("📝 Detected tailwind.css has been modified, restoring...");

    let git_result = Command::new("git")
        .args(&["checkout", "--", "public/tailwind.css"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run git checkout")?;

    if git_result.success() {
        println!("{}", "✅ tailwind.css restored".green().bold());

        // Confirm status again
        let confirm_output = Command::new("git")
            .args(&["status", "--porcelain", "public/tailwind.css"])
            .output()
            .context("Failed to confirm git status")?;

        let confirm_str = String::from_utf8_lossy(&confirm_output.stdout);
        if confirm_str.trim().is_empty() {
            println!("{}", "✅ Restore confirmation succeeded".green());
        } else {
            println!(
                "{}",
                "⚠️  Restore after still has uncommitted changes".yellow()
            );
        }
    } else {
        anyhow::bail!("❌ Failed to restore tailwind.css");
    }

    Ok(())
}

fn upload_to_remote() -> Result<()> {
    println!("\n{}", "🚀 Uploading to remote server...".yellow().bold());

    // Load .env environment variables if present
    if std::path::Path::new(".env").exists() {
        dotenv::dotenv().ok();
    } else {
        println!("ℹ️  No .env file found, using existing environment variables.");
    }

    // Prefer staging-specific variables for quick testing environment deployment.
    let deploy_user = std::env::var("STAGING_DEPLOY_USER")
        .or_else(|_| std::env::var("DEPLOY_USER"))
        .context("❌ Missing STAGING_DEPLOY_USER/DEPLOY_USER environment variable")?;
    let deploy_host = std::env::var("STAGING_DEPLOY_HOST")
        .or_else(|_| std::env::var("DEPLOY_HOST"))
        .context("❌ Missing STAGING_DEPLOY_HOST/DEPLOY_HOST environment variable")?;
    let deploy_path = std::env::var("STAGING_DEPLOY_PATH")
        .or_else(|_| std::env::var("DEPLOY_PATH"))
        .context("❌ Missing STAGING_DEPLOY_PATH/DEPLOY_PATH environment variable")?;
    let ssh_key_file = resolve_ssh_key_file();

    let deploy_target = format!("{}@{}:{}", deploy_user, deploy_host, deploy_path);
    println!("Uploading to: {}", deploy_target);

    let tar_file = "target/dx/ifecaro/release/web/public.tar.gz";
    if !std::path::Path::new(tar_file).exists() {
        anyhow::bail!("❌ Unable to find deployment package public.tar.gz");
    }

    // Ensure remote directory exists
    let ssh_args = &[
        "-i",
        &ssh_key_file,
        "-o",
        "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o",
        "StrictHostKeyChecking=yes",
        "-o",
        "PasswordAuthentication=no",
        "-o",
        "PubkeyAuthentication=yes",
        "-o",
        "ConnectTimeout=30",
        &format!("{}@{}", deploy_user, deploy_host),
        &format!("mkdir -p {}", deploy_path),
    ];

    let mkdir_result = Command::new("ssh")
        .args(ssh_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("SSH connection failed")?;

    if !mkdir_result.success() {
        println!("⚠️  Remote directory creation failed, continuing to try upload");
    }

    // Upload deployment package
    let scp_args = &[
        "-i",
        &ssh_key_file,
        "-o",
        "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o",
        "StrictHostKeyChecking=yes",
        "-o",
        "PasswordAuthentication=no",
        "-o",
        "PubkeyAuthentication=yes",
        "-o",
        "ConnectTimeout=30",
        tar_file,
        &format!("{}@{}:{}/", deploy_user, deploy_host, deploy_path),
    ];

    let upload_result = Command::new("scp")
        .args(scp_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("SCP upload failed")?;

    if !upload_result.success() {
        anyhow::bail!("❌ Deployment package upload failed");
    }

    println!("{}", "✅ Deployment package uploaded".green().bold());

    // Remote decompression
    extract_on_remote(&deploy_user, &deploy_host, &deploy_path, &ssh_key_file)?;

    // Restart remote Docker service
    restart_remote_docker(&deploy_user, &deploy_host, &deploy_path, &ssh_key_file)?;

    println!("{}", "✅ Staging deployment completed".green().bold());
    Ok(())
}

fn run_remote_deploy_binary() -> Result<()> {
    println!(
        "\n{}",
        "🌐 Running remote VPS deployment (minimal standalone binary)..."
            .yellow()
            .bold()
    );

    let status = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            "tools/deploy-remote/Cargo.toml",
            "--release",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run deploy-remote binary")?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("❌ deploy-remote execution failed");
    }
}

fn resolve_ssh_key_file() -> String {
    if let Ok(ssh_key_file) = std::env::var("SSH_KEY_FILE") {
        if !ssh_key_file.trim().is_empty() {
            return ssh_key_file;
        }
    }

    let ssh_key_path = std::env::var("SSH_KEY_PATH").unwrap_or_else(|_| "/root/.ssh".to_string());
    let ssh_key_name = std::env::var("SSH_KEY_NAME").unwrap_or_else(|_| "id_rsa".to_string());
    format!("{}/{}", ssh_key_path, ssh_key_name)
}

fn extract_on_remote(user: &str, host: &str, path: &str, ssh_key_file: &str) -> Result<()> {
    println!("Running remote decompression...");

    let extract_command = format!(
        "cd {} && mkdir -p frontend_new && cd frontend_new && tar -xzf ../public.tar.gz --strip-components=1 && cd .. && rm -rf frontend_old && mv frontend frontend_old 2>/dev/null || true && mv frontend_new frontend",
        path
    );

    let ssh_args = &[
        "-i",
        ssh_key_file,
        "-o",
        "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o",
        "StrictHostKeyChecking=yes",
        "-o",
        "PasswordAuthentication=no",
        "-o",
        "PubkeyAuthentication=yes",
        "-o",
        "ConnectTimeout=30",
        &format!("{}@{}", user, host),
        &extract_command,
    ];

    let extract_result = Command::new("ssh")
        .args(ssh_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run remote decompression")?;

    if extract_result.success() {
        println!("{}", "✅ Remote decompression completed".green().bold());
    } else {
        println!("⚠️  Remote decompression failed, but upload succeeded");
    }

    Ok(())
}

fn restart_remote_docker(user: &str, host: &str, path: &str, ssh_key_file: &str) -> Result<()> {
    println!("Restarting remote Docker service...");

    let restart_command = format!("cd {} && docker compose restart", path);

    let ssh_args = &[
        "-i",
        ssh_key_file,
        "-o",
        "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o",
        "StrictHostKeyChecking=yes",
        "-o",
        "PasswordAuthentication=no",
        "-o",
        "PubkeyAuthentication=yes",
        "-o",
        "ConnectTimeout=30",
        &format!("{}@{}", user, host),
        &restart_command,
    ];

    let restart_result = Command::new("ssh")
        .args(ssh_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to restart remote Docker service")?;

    if restart_result.success() {
        println!("{}", "✅ Remote Docker service restarted".green().bold());
    } else {
        println!("⚠️  Remote Docker service restart failed, but deployment succeeded");
    }

    Ok(())
}

fn clean() -> Result<()> {
    println!("{}", "🧹 Cleaning build files...".yellow().bold());

    let cargo_clean = Command::new("cargo")
        .args(&["clean"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to clean Cargo files")?;

    let dx_clean = Command::new("rm")
        .args(&["-rf", "target/dx"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to clean Dioxus files")?;

    if cargo_clean.success() && dx_clean.success() {
        println!("{}", "✅ Clean completed".green().bold());
    } else {
        anyhow::bail!("❌ Clean failed");
    }

    Ok(())
}

/// Remove heavy debug and incremental caches to shrink target folder size.
fn cleanup_target_artifacts() {
    use std::fs;
    let paths = ["target/debug", "target/release/incremental"];

    for p in &paths {
        if fs::metadata(p).is_ok() {
            if let Err(e) = fs::remove_dir_all(p) {
                eprintln!("Warning: failed to remove {}: {}", p, e);
            }
        }
    }
}

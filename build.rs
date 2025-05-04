use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    // Compile Tailwind CSS
    println!("Compiling Tailwind CSS...");
    let tailwind_status = Command::new("tailwindcss")
        .args(["-m", "-i", "./src/input.css", "-o", "./public/tailwind.css"])
        .status()
        .expect("Failed to execute tailwindcss command");

    if !tailwind_status.success() {
        panic!("Tailwind CSS compilation failed");
    }

    // Execute dx build
    println!("Running dx build...");
    let dx_status = Command::new("dx")
        .args(["build", "--release"])
        .status()
        .expect("Failed to execute dx build command");

    if !dx_status.success() {
        panic!("dx build failed");
    }

    // Copy PWA assets
    println!("Copying PWA assets...");
    let build_dir = Path::new("target/dx/ifecaro/release/web");
    let public_dir = build_dir.join("public");
    
    // Create necessary directories
    let _ = fs::create_dir_all(&public_dir);
    let _ = fs::create_dir_all(public_dir.join("img/icons"));
    
    // Copy files to public directory
    let root_files = [
        ("public/manifest.json", "manifest.json"),
        ("public/sw.js", "sw.js"),
        ("public/img/icons/favicon.ico", "favicon.ico"),
    ];

    for (src, dest) in root_files.iter() {
        if let Err(e) = fs::copy(src, public_dir.join(dest)) {
            println!("Failed to copy {}: {}", src, e);
        }
    }
    
    // Copy icon files to img/icons directory
    let icon_files = [
        "favicon.ico",
        "apple-touch-icon.png",
        "android-chrome-192x192.png",
        "android-chrome-512x512.png",
        "favicon-16x16.png",
        "favicon-32x32.png",
    ];
    
    for icon in icon_files.iter() {
        let _ = fs::copy(
            format!("public/img/icons/{}", icon),
            public_dir.join("img/icons").join(icon),
        );
    }

    // Create tar.gz file (Unix/Linux only)
    if cfg!(unix) {
        println!("Creating tar.gz file...");
        let tar_status = Command::new("tar")
            .args([
                "-czf",
                "public.tar.gz",
                "public/",
            ])
            .current_dir("target/dx/ifecaro/release/web")
            .status()
            .expect("Failed to execute tar command");

        if !tar_status.success() {
            panic!("tar packaging failed");
        }

        // Upload file to server (Unix/Linux only)
        println!("Uploading file to server...");
        let scp_status = Command::new("scp")
            .args([
                "target/dx/ifecaro/release/web/public.tar.gz",
                "togekk@38.242.233.231:~/ifecaro",
            ])
            .status()
            .expect("Failed to execute scp command");

        if !scp_status.success() {
            panic!("File upload failed");
        }
    } else {
        println!("Skipping tar.gz creation and file upload on non-Unix platforms");
    }

    // Restore tailwind.css from git
    println!("Restoring tailwind.css...");
    let git_status = Command::new("git")
        .args(["checkout", "--", "public/tailwind.css"])
        .status()
        .expect("Failed to execute git checkout command");

    if !git_status.success() {
        println!("Warning: Failed to restore tailwind.css");
    }

    println!("All tasks completed successfully!");
} 
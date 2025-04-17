use std::process::Command;
use std::path::Path;
use std::env;

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

    // Create tar.gz file (Unix/Linux only)
    if cfg!(unix) {
        println!("Creating tar.gz file...");
        let tar_status = Command::new("tar")
            .args([
                "-czf",
                "public.tar.gz",
                "public/",
            ])
            .current_dir("target/dx/simon-cheng/release/web")
            .status()
            .expect("Failed to execute tar command");

        if !tar_status.success() {
            panic!("tar packaging failed");
        }

        // Upload file to server (Unix/Linux only)
        println!("Uploading file to server...");
        let scp_status = Command::new("scp")
            .args([
                "target/dx/simon-cheng/release/web/public.tar.gz",
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

    println!("All tasks completed successfully!");
} 
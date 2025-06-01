use std::process::Command;

fn main() {
    // Tell Cargo to re-run build script when these files change
    println!("cargo:rerun-if-changed=src/input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/");
    
    dotenv::dotenv().ok();

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
}
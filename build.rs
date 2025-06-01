use std::process::Command;

fn main() {
    // 告訴 Cargo 當這些文件變更時重新執行 build script
    println!("cargo:rerun-if-changed=src/input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/");
    
    dotenv::dotenv().ok();

    // Compile Tailwind CSS
    println!("cargo:warning=開始編譯 Tailwind CSS...");
    let tailwind_status = Command::new("tailwindcss")
        .args(["-m", "-i", "./src/input.css", "-o", "./public/tailwind.css"])
        .status()
        .expect("Failed to execute tailwindcss command");

    if !tailwind_status.success() {
        panic!("Tailwind CSS compilation failed");
    }

    println!("cargo:warning=Tailwind CSS 編譯完成！");
}
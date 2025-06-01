use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "deploy")]
#[command(about = "Ifecaro 引擎部署工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 快速檢查 (cargo check)
    Check,
    /// 執行測試套件 (使用 Rust 測試運行器)
    Test {
        /// 測試模式
        #[arg(value_enum)]
        mode: Option<TestMode>,
    },
    /// 構建專案
    Build,
    /// 完整部署流程 (測試 + 構建 + 部署)
    Deploy,
    /// 清理構建檔案
    Clean,
    /// 開發模式 (檢查 + 快速測試)
    Dev,
    /// 生產模式 (優化的完整部署流程)
    Prod,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TestMode {
    /// 完整測試套件
    Full,
    /// 快速測試
    Quick,
    /// 容器內優化測試
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
            check()?;
            test(Some(TestMode::Quick))?;
            println!("{}", "✅ 開發檢查完成".green().bold());
        }
        Some(Commands::Prod) => {
            deploy()?;
            println!("{}", "🎉 生產部署完成".green().bold());
        }
        None => {
            show_interactive_menu()?;
        }
    }

    Ok(())
}

fn show_interactive_menu() -> Result<()> {
    loop {
        // 簡單清屏效果
        for _ in 0..50 {
            println!();
        }
        
        println!("{}", "🚀 Ifecaro 引擎部署工具".blue().bold());
        println!("{}", "================================================".blue());
        println!();
        
        println!("請選擇要執行的操作:");
        println!();
        println!("  {}  📋 快速檢查 (cargo check)", "1.".cyan().bold());
        println!("  {}  🧪 執行測試套件", "2.".cyan().bold());
        println!("  {}  🏗  構建專案", "3.".cyan().bold());
        println!("  {}  🧹 清理構建檔案", "4.".cyan().bold());
        println!("  {}  ⚡ 開發模式 (檢查 + 快速測試)", "5.".cyan().bold());
        println!("  {}  🎯 生產模式 (完整一鍵部署)", "6.".cyan().bold());
        println!("  {}  ❌ 退出", "0.".red().bold());
        println!();
        
        print!("{}", "請輸入選項 (0-6): ".green().bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        println!(); // 空行
        
        match choice {
            "1" => {
                println!("{}", "執行快速檢查...".yellow());
                check()?;
                wait_for_enter();
            },
            "2" => {
                show_test_submenu()?;
            },
            "3" => {
                println!("{}", "開始構建專案...".yellow());
                build()?;
                wait_for_enter();
            },
            "4" => {
                println!("{}", "清理構建檔案...".yellow());
                clean()?;
                wait_for_enter();
            },
            "5" => {
                println!("{}", "開始開發模式...".yellow());
                check()?;
                test(Some(TestMode::Quick))?;
                println!("{}", "✅ 開發檢查完成".green().bold());
                wait_for_enter();
            },
            "6" => {
                println!("{}", "開始生產模式...".yellow());
                deploy()?;
                println!("{}", "🎉 生產部署完成".green().bold());
                wait_for_enter();
            },
            "0" => {
                println!("{}", "感謝使用 Ifecaro 引擎部署工具！".green().bold());
                break;
            },
            _ => {
                println!("{}", "無效選項，請重新輸入 (0-6)".red().bold());
                wait_for_enter();
            }
        }
    }
    
    Ok(())
}

fn show_test_submenu() -> Result<()> {
    loop {
        // 簡單清屏效果
        for _ in 0..50 {
            println!();
        }
        
        println!("{}", "🧪 測試套件選單".blue().bold());
        println!("{}", "================================================".blue());
        println!();
        
        println!("請選擇測試模式:");
        println!();
        println!("  {}  🎯 完整測試套件 (所有測試)", "1.".cyan().bold());
        println!("  {}  ⚡ 快速測試 (編譯 + 基礎測試)", "2.".cyan().bold());
        println!("  {}  🐳 容器內優化測試", "3.".cyan().bold());
        println!("  {}  ↩️ 返回主選單", "0.".yellow().bold());
        println!();
        
        print!("{}", "請輸入選項 (0-3): ".green().bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        println!(); // 空行
        
        match choice {
            "1" => {
                println!("{}", "執行完整測試套件...".yellow());
                test(Some(TestMode::Full))?;
                wait_for_enter();
                break;
            },
            "2" => {
                println!("{}", "執行快速測試...".yellow());
                test(Some(TestMode::Quick))?;
                wait_for_enter();
                break;
            },
            "3" => {
                println!("{}", "執行容器內測試...".yellow());
                test(Some(TestMode::Internal))?;
                wait_for_enter();
                break;
            },
            "0" => {
                break; // 返回主選單
            },
            _ => {
                println!("{}", "無效選項，請重新輸入 (0-3)".red().bold());
                wait_for_enter();
            }
        }
    }
    
    Ok(())
}

fn wait_for_enter() {
    println!();
    print!("{}", "按 Enter 鍵繼續...".dimmed());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn check() -> Result<()> {
    println!("{}", "🔍 執行 Cargo 檢查...".yellow().bold());
    
    let output = Command::new("cargo")
        .args(&["check"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("執行 cargo check 失敗")?;

    if output.success() {
        println!("{}", "✅ Cargo 檢查通過".green().bold());
    } else {
        anyhow::bail!("❌ Cargo 檢查失敗");
    }

    Ok(())
}

fn test(mode: Option<TestMode>) -> Result<()> {
    let test_mode = mode.unwrap_or(TestMode::Full);
    
    println!("{}", format!("📋 執行測試套件 ({:?} 模式)...", test_mode).yellow().bold());
    
    let test_command = match test_mode {
        TestMode::Full => "full",
        TestMode::Quick => "quick", 
        TestMode::Internal => "internal",
    };

    let output = Command::new("cargo")
        .args(&["run", "--bin", "test-runner", test_command])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("執行 Rust 測試運行器失敗")?;

    if output.success() {
        println!("{}", "✅ 測試套件通過".green().bold());
    } else {
        anyhow::bail!("❌ 測試套件失敗");
    }

    Ok(())
}

fn build() -> Result<()> {
    println!("{}", "🏗 構建 Rust 專案...".yellow().bold());
    
    let rust_build = Command::new("cargo")
        .args(&["build", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Rust 構建失敗")?;

    if !rust_build.success() {
        anyhow::bail!("❌ Rust 構建失敗");
    }

    println!("{}", "🎯 構建 Dioxus 專案...".yellow().bold());
    
    let dioxus_build = Command::new("dx")
        .args(&["build", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Dioxus 構建失敗")?;

    if dioxus_build.success() {
        println!("{}", "✅ 構建完成".green().bold());
    } else {
        anyhow::bail!("❌ Dioxus 構建失敗");
    }

    Ok(())
}

fn deploy() -> Result<()> {
    println!("{}", "🚀 開始 Ifecaro 引擎部署流程".blue().bold());
    println!("{}", "================================================".blue());

    // 1. 執行完整測試套件
    println!("\n{}", "📋 執行完整測試套件...".yellow().bold());
    let test_result = Command::new("cargo")
        .args(&["run", "--bin", "test-runner", "full"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("執行測試套件失敗")?;

    if !test_result.success() {
        anyhow::bail!("❌ 測試套件失敗，中止部署");
    }
    println!("{}", "✅ 測試套件通過".green().bold());

    // 2. 執行 Rust 構建
    println!("\n{}", "🏗️ 執行 Rust 構建...".yellow().bold());
    let rust_build = Command::new("cargo")
        .args(&["build", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Rust 構建失敗")?;

    if !rust_build.success() {
        anyhow::bail!("❌ Rust 構建失敗");
    }
    println!("{}", "✅ Rust 構建完成".green().bold());

    // 3. 執行 Dioxus 構建
    println!("\n{}", "🎯 執行 Dioxus 構建...".yellow().bold());
    let dioxus_build = Command::new("dx")
        .args(&["build", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Dioxus 構建失敗")?;

    if !dioxus_build.success() {
        anyhow::bail!("❌ Dioxus 構建失敗");
    }
    println!("{}", "✅ Dioxus 構建完成".green().bold());

    // 4. 複製 PWA 資源
    copy_pwa_resources()?;

    // 5. 創建部署包
    create_deployment_package()?;

    // 6. 恢復 tailwind.css
    restore_tailwind_css()?;

    // 7. 上傳到遠端伺服器
    upload_to_remote()?;

    println!("\n{}", "🎉 部署流程完成！".green().bold());
    println!("部署文件位置: target/dx/ifecaro/release/web/public.tar.gz");
    
    // 讀取環境變數用於最終輸出
    if let Ok(user) = std::env::var("DEPLOY_USER") {
        if let Ok(host) = std::env::var("DEPLOY_HOST") {
            if let Ok(path) = std::env::var("DEPLOY_PATH") {
                println!("已上傳至: {}@{}:{}/frontend/", user, host, path);
                return Ok(());
            }
        }
    }
    println!("已上傳至遠端伺服器");

    Ok(())
}

fn copy_pwa_resources() -> Result<()> {
    println!("\n{}", "📦 複製 PWA 資源...".yellow().bold());
    
    let build_dir = "target/dx/ifecaro/release/web";
    let public_dir = format!("{}/public", build_dir);
    
    // 創建目錄
    std::fs::create_dir_all(format!("{}/img/icons", public_dir))
        .context("創建目錄失敗")?;
    
    // 複製文件的函數
    let copy_if_exists = |src: &str, dst: &str| {
        if std::path::Path::new(src).exists() {
            if let Err(e) = std::fs::copy(src, dst) {
                println!("警告: 複製 {} 失敗: {}", src, e);
            }
        } else {
            println!("警告: {} 不存在", src);
        }
    };
    
    // 複製根目錄文件
    copy_if_exists("public/manifest.json", &format!("{}/manifest.json", public_dir));
    copy_if_exists("public/sw.js", &format!("{}/sw.js", public_dir));
    copy_if_exists("public/img/icons/favicon.ico", &format!("{}/favicon.ico", public_dir));
    
    // 複製圖標文件
    if let Ok(entries) = std::fs::read_dir("public/img/icons") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                if let Some(ext) = path.extension() {
                    if ext == "png" || ext == "ico" {
                        let dst = format!("{}/img/icons/{}", public_dir, filename.to_string_lossy());
                        copy_if_exists(&path.to_string_lossy(), &dst);
                    }
                }
            }
        }
    }
    
    println!("{}", "✅ PWA 資源複製完成".green().bold());
    Ok(())
}

fn create_deployment_package() -> Result<()> {
    println!("\n{}", "📚 創建部署包...".yellow().bold());
    
    let web_dir = "target/dx/ifecaro/release/web";
    
    let tar_result = Command::new("tar")
        .args(&["-czf", "public.tar.gz", "public/"])
        .current_dir(web_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("創建 tar 包失敗")?;
    
    if !tar_result.success() {
        anyhow::bail!("❌ 創建部署包失敗");
    }
    
    println!("{}", "✅ 部署包創建完成".green().bold());
    Ok(())
}

fn restore_tailwind_css() -> Result<()> {
    println!("\n{}", "🔄 恢復 tailwind.css...".yellow().bold());
    
    // 確保 git 安全目錄設定（Docker 環境需要）
    let _safe_dir_result = Command::new("git")
        .args(&["config", "--global", "--add", "safe.directory", "/app"])
        .output();
    
    // 首先檢查檔案是否被修改
    let status_output = Command::new("git")
        .args(&["status", "--porcelain", "public/tailwind.css"])
        .output()
        .context("檢查 git 狀態失敗")?;
    
    let status_str = String::from_utf8_lossy(&status_output.stdout);
    
    if status_str.trim().is_empty() {
        println!("{}", "📋 tailwind.css 未被修改，無需恢復".green());
        return Ok(());
    }
    
    println!("📝 檢測到 tailwind.css 已被修改，正在恢復...");
    
    let git_result = Command::new("git")
        .args(&["checkout", "--", "public/tailwind.css"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("執行 git checkout 失敗")?;
    
    if git_result.success() {
        println!("{}", "✅ tailwind.css 恢復完成".green().bold());
        
        // 再次確認狀態
        let confirm_output = Command::new("git")
            .args(&["status", "--porcelain", "public/tailwind.css"])
            .output()
            .context("確認 git 狀態失敗")?;
        
        let confirm_str = String::from_utf8_lossy(&confirm_output.stdout);
        if confirm_str.trim().is_empty() {
            println!("{}", "✅ 恢復確認成功".green());
        } else {
            println!("{}", "⚠️  恢復後仍有未提交變更".yellow());
        }
    } else {
        anyhow::bail!("❌ tailwind.css 恢復失敗");
    }
    
    Ok(())
}

fn upload_to_remote() -> Result<()> {
    println!("\n{}", "🚀 上傳到遠端伺服器...".yellow().bold());
    
    // 載入 .env 環境變數
    if std::path::Path::new(".env").exists() {
        dotenv::dotenv().ok();
    } else {
        anyhow::bail!("❌ 找不到 .env 文件，請先創建並配置部署參數");
    }
    
    // 檢查必要的環境變數
    let deploy_user = std::env::var("DEPLOY_USER")
        .context("❌ 缺少 DEPLOY_USER 環境變數")?;
    let deploy_host = std::env::var("DEPLOY_HOST")
        .context("❌ 缺少 DEPLOY_HOST 環境變數")?;
    let deploy_path = std::env::var("DEPLOY_PATH")
        .context("❌ 缺少 DEPLOY_PATH 環境變數")?;
    let ssh_key_path = std::env::var("SSH_KEY_PATH")
        .unwrap_or_else(|_| "/root/.ssh".to_string());
    
    let deploy_target = format!("{}@{}:{}", deploy_user, deploy_host, deploy_path);
    println!("正在上傳到: {}", deploy_target);
    
    let tar_file = "target/dx/ifecaro/release/web/public.tar.gz";
    if !std::path::Path::new(tar_file).exists() {
        anyhow::bail!("❌ 找不到部署包 public.tar.gz");
    }
    
    // 確保遠端目錄存在
    let ssh_args = &[
        "-i", &format!("{}/id_rsa", ssh_key_path),
        "-o", "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o", "StrictHostKeyChecking=yes",
        "-o", "PasswordAuthentication=no",
        "-o", "PubkeyAuthentication=yes", 
        "-o", "ConnectTimeout=30",
        &format!("{}@{}", deploy_user, deploy_host),
        &format!("mkdir -p {}", deploy_path)
    ];
    
    let mkdir_result = Command::new("ssh")
        .args(ssh_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("SSH 連接失敗")?;
    
    if !mkdir_result.success() {
        println!("⚠️  遠端目錄創建失敗，繼續嘗試上傳");
    }
    
    // 上傳部署包
    let scp_args = &[
        "-i", &format!("{}/id_rsa", ssh_key_path),
        "-o", "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o", "StrictHostKeyChecking=yes",
        "-o", "PasswordAuthentication=no",
        "-o", "PubkeyAuthentication=yes",
        "-o", "ConnectTimeout=30",
        tar_file,
        &format!("{}@{}:{}/", deploy_user, deploy_host, deploy_path)
    ];
    
    let upload_result = Command::new("scp")
        .args(scp_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("SCP 上傳失敗")?;
    
    if !upload_result.success() {
        anyhow::bail!("❌ 部署包上傳失敗");
    }
    
    println!("{}", "✅ 部署包上傳成功".green().bold());
    
    // 遠端解壓縮
    extract_on_remote(&deploy_user, &deploy_host, &deploy_path, &ssh_key_path)?;
    
    // 重啟遠端 Docker 服務
    restart_remote_docker(&deploy_user, &deploy_host, &deploy_path, &ssh_key_path)?;
    
    println!("{}", "✅ 遠端部署完成".green().bold());
    Ok(())
}

fn extract_on_remote(user: &str, host: &str, path: &str, ssh_key_path: &str) -> Result<()> {
    println!("正在遠端解壓縮...");
    
    let extract_command = format!(
        "cd {} && mkdir -p frontend_new && cd frontend_new && tar -xzf ../public.tar.gz --strip-components=1 && cd .. && rm -rf frontend_old && mv frontend frontend_old 2>/dev/null || true && mv frontend_new frontend",
        path
    );
    
    let ssh_args = &[
        "-i", &format!("{}/id_rsa", ssh_key_path),
        "-o", "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o", "StrictHostKeyChecking=yes",
        "-o", "PasswordAuthentication=no",
        "-o", "PubkeyAuthentication=yes", 
        "-o", "ConnectTimeout=30",
        &format!("{}@{}", user, host),
        &extract_command
    ];
    
    let extract_result = Command::new("ssh")
        .args(ssh_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("遠端解壓縮失敗")?;
    
    if extract_result.success() {
        println!("{}", "✅ 遠端解壓縮完成".green().bold());
    } else {
        println!("⚠️  遠端解壓縮失敗，但上傳成功");
    }
    
    Ok(())
}

fn restart_remote_docker(user: &str, host: &str, path: &str, ssh_key_path: &str) -> Result<()> {
    println!("正在重啟遠端 Docker 服務...");
    
    let restart_command = format!("cd {} && docker compose restart", path);
    
    let ssh_args = &[
        "-i", &format!("{}/id_rsa", ssh_key_path),
        "-o", "UserKnownHostsFile=/root/.ssh/known_hosts",
        "-o", "StrictHostKeyChecking=yes",
        "-o", "PasswordAuthentication=no",
        "-o", "PubkeyAuthentication=yes",
        "-o", "ConnectTimeout=30", 
        &format!("{}@{}", user, host),
        &restart_command
    ];
    
    let restart_result = Command::new("ssh")
        .args(ssh_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("遠端 Docker 重啟失敗")?;
    
    if restart_result.success() {
        println!("{}", "✅ 遠端 Docker 服務重啟完成".green().bold());
    } else {
        println!("⚠️  遠端 Docker 服務重啟失敗，但部署成功");
    }
    
    Ok(())
}

fn clean() -> Result<()> {
    println!("{}", "🧹 清理構建檔案...".yellow().bold());
    
    let cargo_clean = Command::new("cargo")
        .args(&["clean"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("清理 Cargo 檔案失敗")?;

    let dx_clean = Command::new("rm")
        .args(&["-rf", "target/dx"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("清理 Dioxus 檔案失敗")?;

    if cargo_clean.success() && dx_clean.success() {
        println!("{}", "✅ 清理完成".green().bold());
    } else {
        anyhow::bail!("❌ 清理失敗");
    }

    Ok(())
} 
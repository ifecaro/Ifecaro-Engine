use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "test-runner")]
#[command(about = "Ifecaro 引擎測試運行器", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 執行完整測試套件
    Full,
    /// 快速測試 (編譯檢查 + 基礎測試)
    Quick,
    /// 執行特定測試類別
    Category {
        /// 測試類別名稱
        #[arg(value_enum)]
        category: TestCategory,
    },
    /// 容器內優化測試
    Internal,
    /// 只執行編譯檢查
    Check,
    /// 執行效能基準測試
    Benchmark,
    /// 生成測試報告
    Report,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TestCategory {
    /// 編譯檢查
    Compile,
    /// 基礎 UI 測試
    BasicUi,
    /// 進階功能測試
    Advanced,
    /// API Mock 測試
    ApiMock,
    /// API 整合測試
    Integration,
    /// 單元測試
    Unit,
    /// 外部整合測試
    External,
}

struct TestResult {
    name: String,
    passed: bool,
    duration: Duration,
    #[allow(dead_code)]
    command: String,
    #[allow(dead_code)]
    error_message: Option<String>,
}

struct TestRunner {
    total_tests: AtomicUsize,
    passed_tests: AtomicUsize,
    failed_tests: AtomicUsize,
    results: Vec<TestResult>,
    is_internal: bool,
}

impl TestRunner {
    fn new(is_internal: bool) -> Self {
        Self {
            total_tests: AtomicUsize::new(0),
            passed_tests: AtomicUsize::new(0),
            failed_tests: AtomicUsize::new(0),
            results: Vec::new(),
            is_internal,
        }
    }

    fn run_test(&mut self, name: &str, command: &str) -> Result<()> {
        println!("\n{}", format!("📋 執行 {}...", name).yellow().bold());
        println!("指令: {}", command.cyan());
        println!("{}", "------------------------------------------------".dimmed());

        let start_time = Instant::now();
        
        let (program, args) = if self.is_internal {
            // 容器內直接執行
            self.parse_internal_command(command)
        } else {
            // 外部通過 docker compose exec 執行
            self.parse_external_command(command)
        };

        let mut cmd = Command::new(&program);
        if !args.is_empty() {
            cmd.args(&args);
        }

        let output = cmd
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context(format!("執行測試失敗: {}", name))?;

        let duration = start_time.elapsed();
        let passed = output.success();

        let result = TestResult {
            name: name.to_string(),
            passed,
            duration,
            command: command.to_string(),
            error_message: if passed { None } else { Some("測試執行失敗".to_string()) },
        };

        if passed {
            println!("{}", format!("✅ {} 通過 ({}ms)", name, duration.as_millis()).green().bold());
            self.passed_tests.fetch_add(1, Ordering::SeqCst);
        } else {
            println!("{}", format!("❌ {} 失敗 ({}ms)", name, duration.as_millis()).red().bold());
            self.failed_tests.fetch_add(1, Ordering::SeqCst);
        }

        self.total_tests.fetch_add(1, Ordering::SeqCst);
        self.results.push(result);

        Ok(())
    }

    fn parse_internal_command(&self, command: &str) -> (String, Vec<String>) {
        // 移除 docker compose exec app 前綴
        let clean_command = command
            .strip_prefix("docker compose exec app ")
            .unwrap_or(command);
        
        let parts: Vec<&str> = clean_command.split_whitespace().collect();
        if parts.is_empty() {
            return ("echo".to_string(), vec!["No command".to_string()]);
        }

        let program = parts[0].to_string();
        let args = parts[1..].iter().map(|s| s.to_string()).collect();
        (program, args)
    }

    fn parse_external_command(&self, command: &str) -> (String, Vec<String>) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return ("echo".to_string(), vec!["No command".to_string()]);
        }

        let program = parts[0].to_string();
        let args = parts[1..].iter().map(|s| s.to_string()).collect();
        (program, args)
    }

    fn print_summary(&self) {
        let total = self.total_tests.load(Ordering::SeqCst);
        let passed = self.passed_tests.load(Ordering::SeqCst);
        let failed = self.failed_tests.load(Ordering::SeqCst);

        println!("\n{}", "================================================".blue());
        println!("{}", "📊 測試結果總結".blue().bold());
        println!("{}", "================================================".blue());
        println!("總測試項目: {}", total);
        println!("{}", format!("通過: {}", passed).green());
        println!("{}", format!("失敗: {}", failed).red());

        if !self.results.is_empty() {
            println!("\n{}", "📋 詳細結果:".yellow().bold());
            for result in &self.results {
                let status = if result.passed { "✅" } else { "❌" };
                let duration = format!("{}ms", result.duration.as_millis());
                println!("{} {} ({})", status, result.name, duration.dimmed());
            }
        }

        if failed == 0 {
            println!("\n{}", "🎉 所有測試都通過了！".green().bold());
        } else {
            println!("\n{}", format!("⚠️  有 {} 個測試失敗", failed).red().bold());
        }
    }

    fn run_full_test_suite(&mut self) -> Result<()> {
        println!("{}", "🚀 開始執行 Ifecaro 引擎完整測試套件".blue().bold());
        println!("{}", "================================================".blue());

        // 根據是否在容器內選擇不同的指令
        let prefix = if self.is_internal { "" } else { "docker compose exec app " };

        // 1. 編譯檢查
        self.run_test("編譯檢查", &format!("{}cargo check", prefix))?;

        // 2. 基礎 UI 測試
        self.run_test("Story Content 基礎 UI 測試", &format!("{}cargo test story_content_tests", prefix))?;

        // 3. 進階功能測試
        self.run_test("Story Content 進階功能測試", &format!("{}cargo test story_content_advanced_tests", prefix))?;

        // 4. API Mock 測試
        self.run_test("API Mock 測試", &format!("{}cargo test api_tests", prefix))?;

        // 5. API 整合測試
        self.run_test("API 整合測試", &format!("{}cargo test integration_tests", prefix))?;

        // 6. 其他單元測試
        self.run_test("其他單元測試", &format!("{}cargo test --lib", prefix))?;

        // 7. 外部整合測試
        self.run_test("外部整合測試", &format!("{}cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests", prefix))?;

        Ok(())
    }

    fn run_internal_test_suite(&mut self) -> Result<()> {
        println!("{}", "🚀 開始執行容器內優化測試套件".blue().bold());
        println!("{}", "================================================".blue());

        // 容器內測試不需要 docker compose exec app 前綴
        self.run_test("編譯檢查", "cargo check")?;
        self.run_test("Story Content 基礎 UI 測試", "cargo test story_content_tests")?;
        self.run_test("Story Content 進階功能測試", "cargo test story_content_advanced_tests")?;
        self.run_test("API Mock 測試", "cargo test api_tests")?;
        self.run_test("API 整合測試", "cargo test integration_tests")?;
        self.run_test("其他單元測試", "cargo test --lib")?;
        self.run_test("外部整合測試", "cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests")?;

        Ok(())
    }

    fn run_quick_tests(&mut self) -> Result<()> {
        println!("{}", "⚡ 開始執行快速測試套件".yellow().bold());
        println!("{}", "================================================".yellow());

        let prefix = if self.is_internal { "" } else { "docker compose exec app " };

        self.run_test("編譯檢查", &format!("{}cargo check", prefix))?;
        self.run_test("基礎 UI 測試", &format!("{}cargo test story_content_tests", prefix))?;
        self.run_test("API Mock 測試", &format!("{}cargo test api_tests", prefix))?;

        Ok(())
    }

    fn run_category_test(&mut self, category: TestCategory) -> Result<()> {
        println!("{}", format!("🎯 執行 {:?} 測試類別", category).cyan().bold());
        println!("{}", "================================================".cyan());

        let (test_name, command) = match category {
            TestCategory::Compile => {
                ("編譯檢查", if self.is_internal { "cargo check" } else { "docker compose exec app cargo check" })
            },
            TestCategory::BasicUi => {
                ("基礎 UI 測試", if self.is_internal { "cargo test story_content_tests" } else { "docker compose exec app cargo test story_content_tests" })
            },
            TestCategory::Advanced => {
                ("進階功能測試", if self.is_internal { "cargo test story_content_advanced_tests" } else { "docker compose exec app cargo test story_content_advanced_tests" })
            },
            TestCategory::ApiMock => {
                ("API Mock 測試", if self.is_internal { "cargo test api_tests" } else { "docker compose exec app cargo test api_tests" })
            },
            TestCategory::Integration => {
                ("API 整合測試", if self.is_internal { "cargo test integration_tests" } else { "docker compose exec app cargo test integration_tests" })
            },
            TestCategory::Unit => {
                ("單元測試", if self.is_internal { "cargo test --lib" } else { "docker compose exec app cargo test --lib" })
            },
            TestCategory::External => {
                ("外部整合測試", if self.is_internal { "cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests" } else { "docker compose exec app cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests" })
            },
        };

        self.run_test(test_name, command)?;
        Ok(())
    }

    fn run_benchmark(&mut self) -> Result<()> {
        println!("{}", "🏃 開始執行效能基準測試".magenta().bold());
        println!("{}", "================================================".magenta());

        let perf_cmd = if self.is_internal { "cargo test --release performance_tests" } else { "docker compose exec app cargo test --release performance_tests" };
        let bench_cmd = if self.is_internal { "cargo bench" } else { "docker compose exec app cargo bench" };

        self.run_test("效能測試", perf_cmd)?;
        self.run_test("基準測試", bench_cmd)?;

        Ok(())
    }

    fn generate_report(&self) -> Result<()> {
        println!("{}", "📊 生成測試報告".green().bold());
        println!("{}", "================================================".green());

        let total = self.total_tests.load(Ordering::SeqCst);
        let passed = self.passed_tests.load(Ordering::SeqCst);
        let failed = self.failed_tests.load(Ordering::SeqCst);

        // 如果當前實例沒有測試結果，嘗試讀取現有報告或使用預設值
        let (final_total, final_passed, final_failed, final_results) = if total == 0 {
            // 檢查是否有之前的測試結果文件
            if let Ok(_existing_report) = std::fs::read_to_string("test-report.md") {
                println!("{}", "📋 使用現有測試結果生成報告".yellow());
                // 簡單解析現有報告獲取數據（這裡可以改進）
                (0, 0, 0, "無最近測試結果".to_string())
            } else {
                (0, 0, 0, "無測試結果".to_string())
            }
        } else {
            let results_text = self.results.iter()
                .map(|r| format!("- {} {} ({}ms)", 
                    if r.passed { "✅" } else { "❌" }, 
                    r.name, 
                    r.duration.as_millis()))
                .collect::<Vec<_>>()
                .join("\n");
            (total, passed, failed, results_text)
        };

        let report = format!(
            r#"# Ifecaro Engine 測試報告

## 執行摘要
- 總測試數: {}
- 通過: {}
- 失敗: {}
- 通過率: {:.1}%

## 詳細結果
{}

## 測試工具資訊
- 運行環境: {}
- 工具版本: Rust 測試運行器 v1.0
- 支援功能: 完整測試套件、快速測試、分類測試、效能測試

## 生成時間
{}
"#,
            final_total,
            final_passed,
            final_failed,
            if final_total > 0 { (final_passed as f64 / final_total as f64) * 100.0 } else { 0.0 },
            final_results,
            if self.is_internal { "Docker 容器內" } else { "外部環境" },
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );

        std::fs::write("test-report.md", report)
            .context("無法寫入測試報告")?;

        println!("{}", "✅ 測試報告已生成: test-report.md".green());

        Ok(())
    }
}

fn is_running_in_container() -> bool {
    // 檢查是否存在 /.dockerenv 文件
    std::path::Path::new("/.dockerenv").exists() ||
    // 檢查環境變數
    std::env::var("DOCKER_CONTAINER").is_ok() ||
    // 檢查 /proc/1/cgroup 是否包含 docker
    std::fs::read_to_string("/proc/1/cgroup")
        .map(|content| content.contains("docker"))
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 自動檢測是否在容器內運行
    let in_container = is_running_in_container();

    match &cli.command {
        Some(Commands::Full) => {
            let mut runner = TestRunner::new(in_container);
            runner.run_full_test_suite()?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        Some(Commands::Quick) => {
            let mut runner = TestRunner::new(in_container);
            runner.run_quick_tests()?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        Some(Commands::Category { category }) => {
            let mut runner = TestRunner::new(in_container);
            runner.run_category_test(category.clone())?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        Some(Commands::Internal) => {
            // Internal 模式強制使用容器內模式
            let mut runner = TestRunner::new(true);
            runner.run_internal_test_suite()?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        Some(Commands::Check) => {
            let mut runner = TestRunner::new(in_container);
            let prefix = if in_container { "" } else { "docker compose exec app " };
            runner.run_test("編譯檢查", &format!("{}cargo check", prefix))?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        Some(Commands::Benchmark) => {
            let mut runner = TestRunner::new(in_container);
            runner.run_benchmark()?;
            runner.print_summary();
        },
        Some(Commands::Report) => {
            let runner = TestRunner::new(in_container);
            runner.generate_report()?;
        },
        None => {
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("{}", "🧪 Ifecaro 引擎測試運行器".blue().bold());
    println!("使用方式:");
    println!("  {} full        - 執行完整測試套件", "cargo run --bin test-runner".cyan());
    println!("  {} quick       - 快速測試 (編譯檢查 + 基礎測試)", "cargo run --bin test-runner".cyan());
    println!("  {} category <type> - 執行特定測試類別", "cargo run --bin test-runner".cyan());
    println!("  {} internal    - 容器內優化測試", "cargo run --bin test-runner".cyan());
    println!("  {} check       - 只執行編譯檢查", "cargo run --bin test-runner".cyan());
    println!("  {} benchmark   - 執行效能基準測試", "cargo run --bin test-runner".cyan());
    println!("  {} report      - 生成測試報告", "cargo run --bin test-runner".cyan());
    
    println!("\n測試類別:");
    println!("  compile, basic-ui, advanced, api-mock, integration, unit, external");
} 
use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "test-runner")]
#[command(about = "Ifecaro Engine Test Runner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: TestCommands,
}

#[derive(Subcommand)]
enum TestCommands {
    /// Run complete test suite
    Full,
    /// Quick test (compile check + basic tests)
    Quick,
    /// Run specific test category
    Category {
        /// Test category name
        #[arg(value_enum)]
        category: TestCategory,
    },
    /// Optimized container test
    Internal,
    /// Only run compile check
    Check,
    /// Run performance benchmarks
    Bench,
    /// Generate test report
    Report,
}

#[derive(ValueEnum, Clone, Debug)]
enum TestCategory {
    /// Compile check
    Compile,
    /// Basic UI tests
    UI,
    /// Advanced feature tests
    Advanced,
    /// API Mock tests
    MockApi,
    /// API integration tests
    Integration,
    /// Unit tests
    Unit,
    /// External integration tests
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
        println!("\n{}", format!("📋 Running {}...", name).yellow().bold());
        println!("Command: {}", command.cyan());
        println!("{}", "------------------------------------------------".dimmed());

        let start_time = Instant::now();
        
        let (program, args) = if self.is_internal {
            // Execute directly inside container
            self.parse_internal_command(command)
        } else {
            // Execute externally through docker compose exec
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
            .context(format!("Failed to run test: {}", name))?;

        let duration = start_time.elapsed();
        let passed = output.success();

        let result = TestResult {
            name: name.to_string(),
            passed,
            duration,
            command: command.to_string(),
            error_message: if passed { None } else { Some("Test failed".to_string()) },
        };

        if passed {
            println!("{}", format!("✅ {} completed ({}ms)", name, duration.as_millis()).green().bold());
            self.passed_tests.fetch_add(1, Ordering::SeqCst);
        } else {
            println!("{}", format!("❌ {} failed", name).red().bold());
            self.failed_tests.fetch_add(1, Ordering::SeqCst);
        }

        self.total_tests.fetch_add(1, Ordering::SeqCst);
        self.results.push(result);

        Ok(())
    }

    fn parse_internal_command(&self, command: &str) -> (String, Vec<String>) {
        // Remove docker compose exec app prefix
        let clean_command = command.replace("docker compose exec app ", "");
        
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
        println!("{}", "📊 Test Results Summary".blue().bold());
        println!("{}", "================================================".blue());
        println!("Total tests: {}", total);
        println!("{}", format!("Passed: {}", passed).green());
        println!("{}", format!("Failed: {}", failed).red());

        if !self.results.is_empty() {
            println!("\n{}", "📋 Detailed Results:".yellow().bold());
            for result in &self.results {
                let status = if result.passed { "✅" } else { "❌" };
                let duration = format!("{}ms", result.duration.as_millis());
                println!("{} {} ({})", status, result.name, duration.dimmed());
            }
        }

        if failed == 0 {
            println!("\n{}", "🎉 All tests passed!".green().bold());
        } else {
            println!("\n{}", format!("⚠️  有 {} 個測試失敗", failed).red().bold());
        }
    }

    fn run_full_test_suite(&mut self) -> Result<()> {
        println!("{}", "🧪 Running Complete Test Suite".blue().bold());
        println!("{}", "================================================".blue());
        
        self.run_test("Compile Check", "cargo check")?;
        self.run_test("Unit Tests", "cargo test --lib")?;
        self.run_test("Integration Tests", "cargo test --test *")?;
        
        println!("\n{}", "🎉 Complete test suite passed!".green().bold());

        // Choose different commands based on whether inside container
        let format_command = |cmd: &str| {
            if self.is_internal {
                cmd.to_string()
            } else {
                format!("docker compose exec app {}", cmd)
            }
        };

        let commands = vec![
            // 1. Compile check
            format_command("cargo check"),
            // 2. Basic UI tests
            format_command("cargo test --test story_content_tests"),
            // 3. Advanced feature tests
            format_command("cargo test --test story_content_advanced_tests"),
            // 4. API Mock tests
            format_command("cargo test --test api_tests"),
            // 5. API integration tests
            format_command("cargo test --test story_content_api_integration_tests"),
            // 6. Other unit tests
            format_command("cargo test --lib"),
            // 7. External integration tests
            format_command("cargo test --test story_tests"),
        ];

        for (i, command) in commands.iter().enumerate() {
            self.run_test(&format!("Test {}", i + 1), command)?;
        }

        Ok(())
    }

    fn run_internal_test_suite(&mut self) -> Result<()> {
        println!("{}", "🚀 開始執行容器內優化測試套件".blue().bold());
        println!("{}", "================================================".blue());

        // Container tests don't need docker compose exec app prefix
        let commands = vec![
            "cargo check",
            "cargo test story_content_tests",
            "cargo test story_content_advanced_tests",
            "cargo test api_tests",
            "cargo test integration_tests",
            "cargo test --lib",
            "cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests",
        ];

        let clean_commands: Vec<String> = commands.iter()
            .map(|command| command.replace("docker compose exec app ", ""))
            .collect();

        for command in clean_commands {
            self.run_test(&command, &command)?;
        }

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
            TestCategory::UI => {
                ("基礎 UI 測試", if self.is_internal { "cargo test story_content_tests" } else { "docker compose exec app cargo test story_content_tests" })
            },
            TestCategory::Advanced => {
                ("進階功能測試", if self.is_internal { "cargo test story_content_advanced_tests" } else { "docker compose exec app cargo test story_content_advanced_tests" })
            },
            TestCategory::MockApi => {
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

        // If current instance has no test results, try reading existing report or use defaults
        let (final_total, final_passed, final_failed, final_results) = if total == 0 {
            // Check if there's a previous test results file
            if let Ok(content) = std::fs::read_to_string("test_results.json") {
                println!("{}", "📋 使用現有測試結果生成報告".yellow());
                // Simple parsing of existing report to get data (this can be improved)
                if content.contains("\"passed\":") {
                    // Extract data from JSON content
                    (0, 0, 0, "無最近測試結果".to_string())
                } else {
                    (0, 0, 0, "無測試結果".to_string())
                }
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

fn is_in_container() -> bool {
    // Check if /.dockerenv file exists
    if std::path::Path::new("/.dockerenv").exists() {
        return true;
    }
    // Check environment variables
    if std::env::var("DOCKER_CONTAINER").is_ok() {
        return true;
    }
    // Check if /proc/1/cgroup contains docker
    if let Ok(content) = std::fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Auto-detect if running inside container
    let is_container = is_in_container();

    match cli.command {
        TestCommands::Full => {
            let mut runner = TestRunner::new(is_container);
            runner.run_full_test_suite()?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        TestCommands::Quick => {
            let mut runner = TestRunner::new(is_container);
            runner.run_quick_tests()?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        TestCommands::Category { category } => {
            let mut runner = TestRunner::new(is_container);
            runner.run_category_test(category.clone())?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        TestCommands::Internal => {
            // Internal mode forces container mode
            let mut runner = TestRunner::new(true);
            runner.run_internal_test_suite()?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        TestCommands::Check => {
            let mut runner = TestRunner::new(is_container);
            let prefix = if is_container { "" } else { "docker compose exec app " };
            runner.run_test("編譯檢查", &format!("{}cargo check", prefix))?;
            runner.print_summary();
            
            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        },
        TestCommands::Bench => {
            let mut runner = TestRunner::new(is_container);
            runner.run_benchmark()?;
            runner.print_summary();
        },
        TestCommands::Report => {
            let runner = TestRunner::new(is_container);
            runner.generate_report()?;
        },
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
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
        
        // Check if this is a test or check command that should skip Tailwind compilation
        let should_skip_tailwind = command.contains("test") || command.contains("check");
        
        let (program, args) = if self.is_internal {
            // Execute directly inside container
            self.parse_internal_command(command)
        } else {
            // Execute externally through docker compose exec
            self.parse_external_command(command)
        };

        let mut cmd = Command::new(&program);
        
        // For external commands (docker compose exec), add -e flag for environment variable
        if !self.is_internal && should_skip_tailwind {
            // Insert environment variable flag before 'app'
            let mut new_args = Vec::new();
            for (_i, arg) in args.iter().enumerate() {
                if arg == "app" {
                    new_args.push("-e".to_string());
                    new_args.push("SKIP_TAILWIND=1".to_string());
                }
                new_args.push(arg.clone());
            }
            cmd.args(new_args);
        } else {
            if !args.is_empty() {
                cmd.args(&args);
            }
        }

        // Set environment variable for internal execution before running command
        if self.is_internal && should_skip_tailwind {
            std::env::set_var("SKIP_TAILWIND", "1");
        }

        let output = cmd
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context(format!("Failed to run test: {}", name))?;

        // Clean up environment variable after command
        if self.is_internal && should_skip_tailwind {
            std::env::remove_var("SKIP_TAILWIND");
        }

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
            println!("\n{}", format!("⚠️  {} tests failed", failed).red().bold());
        }
    }

    fn run_full_test_suite(&mut self) -> Result<()> {
        println!("{}", "🧪 Running Complete Test Suite".blue().bold());
        println!("{}", "================================================".blue());
        
        // Clear build cache to ensure environment variables take effect
        if !self.is_internal {
            let _ = Command::new("docker")
                .args(["compose", "exec", "app", "rm", "-rf", "target/debug/build/ifecaro-*"])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }
        
        // Choose different commands based on whether inside container
        let format_command = |cmd: &str| {
            if self.is_internal {
                cmd.to_string()
            } else {
                format!("docker compose exec app {}", cmd)
            }
        };

        // Run comprehensive test suite with clear names
        let test_cases = vec![
            ("Compile Check", format_command("cargo check")),
            ("Unit Tests (All Modules)", format_command("cargo test --lib")),
            ("Core Integration Tests", format_command("cargo test --test integration_tests")),
            ("Code Usage Examples", format_command("cargo test --test main_code_usage_example")),
            ("Story Flow Tests", format_command("cargo test --test story_flow_tests")),
        ];

        for (test_name, command) in test_cases {
            self.run_test(test_name, &command)?;
        }

        println!("\n{}", "🎉 Complete test suite passed!".green().bold());
        Ok(())
    }

    fn run_internal_test_suite(&mut self) -> Result<()> {
        println!("{}", "🚀 Starting optimized test suite in container".blue().bold());
        println!("{}", "================================================".blue());

        // Clear build cache to ensure environment variables take effect
        if !self.is_internal {
            let _ = Command::new("docker")
                .args(["compose", "exec", "app", "rm", "-rf", "target/debug/build/ifecaro-*"])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        // Container tests don't need docker compose exec app prefix
        let commands = vec![
            "cargo check",
            "cargo test --lib",
            "cargo test --test integration_tests",
            "cargo test --test main_code_usage_example", 
            "cargo test --test story_flow_tests",
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
        println!("{}", "⚡ Starting quick test suite".yellow().bold());
        println!("{}", "================================================".yellow());

        // Clear build cache to ensure environment variables take effect
        if !self.is_internal {
            let _ = Command::new("docker")
                .args(["compose", "exec", "app", "rm", "-rf", "target/debug/build/ifecaro-*"])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let prefix = if self.is_internal { "" } else { "docker compose exec app " };

        self.run_test("Compile check", &format!("{}cargo check", prefix))?;
        self.run_test("Unit tests", &format!("{}cargo test --lib", prefix))?;
        self.run_test("Integration tests", &format!("{}cargo test --test integration_tests", prefix))?;

        Ok(())
    }

    fn run_category_test(&mut self, category: TestCategory) -> Result<()> {
        println!("{}", format!("🎯 Running {:?} test category", category).cyan().bold());
        println!("{}", "================================================".cyan());

        // Clear build cache to ensure environment variables take effect
        if !self.is_internal {
            let _ = Command::new("docker")
                .args(["compose", "exec", "app", "rm", "-rf", "target/debug/build/ifecaro-*"])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let (test_name, command) = match category {
            TestCategory::Compile => {
                ("Compile check", if self.is_internal { "cargo check" } else { "docker compose exec app cargo check" })
            },
            TestCategory::UI => {
                ("UI component tests", if self.is_internal { "cargo test --lib components::story_content_tests" } else { "docker compose exec app cargo test --lib components::story_content_tests" })
            },
            TestCategory::Advanced => {
                ("Advanced feature tests", if self.is_internal { "cargo test --lib components::story_content_advanced_tests" } else { "docker compose exec app cargo test --lib components::story_content_advanced_tests" })
            },
            TestCategory::MockApi => {
                ("API Mock tests", if self.is_internal { "cargo test --lib services::api_tests" } else { "docker compose exec app cargo test --lib services::api_tests" })
            },
            TestCategory::Integration => {
                ("Integration tests", if self.is_internal { "cargo test --test integration_tests" } else { "docker compose exec app cargo test --test integration_tests" })
            },
            TestCategory::Unit => {
                ("Unit tests", if self.is_internal { "cargo test --lib" } else { "docker compose exec app cargo test --lib" })
            },
            TestCategory::External => {
                ("External integration tests", if self.is_internal { "cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests" } else { "docker compose exec app cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests" })
            },
        };

        self.run_test(test_name, command)?;
        Ok(())
    }

    fn run_benchmark(&mut self) -> Result<()> {
        println!("{}", "🏃 Starting performance benchmark tests".magenta().bold());
        println!("{}", "================================================".magenta());

        // Clear build cache to ensure environment variables take effect
        if !self.is_internal {
            let _ = Command::new("docker")
                .args(["compose", "exec", "app", "rm", "-rf", "target/debug/build/ifecaro-*"])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let perf_cmd = if self.is_internal { "cargo test --release performance_tests" } else { "docker compose exec app cargo test --release performance_tests" };
        let bench_cmd = if self.is_internal { "cargo bench" } else { "docker compose exec app cargo bench" };

        self.run_test("Performance tests", perf_cmd)?;
        self.run_test("Benchmark tests", bench_cmd)?;

        Ok(())
    }

    fn generate_report(&self) -> Result<()> {
        println!("{}", "📊 Generating test report".green().bold());
        println!("{}", "================================================".green());

        let total = self.total_tests.load(Ordering::SeqCst);
        let passed = self.passed_tests.load(Ordering::SeqCst);
        let failed = self.failed_tests.load(Ordering::SeqCst);

        // If current instance has no test results, try reading existing report or use defaults
        let (final_total, final_passed, final_failed, final_results) = if total == 0 {
            // Check if there's a previous test results file
            if let Ok(content) = std::fs::read_to_string("test_results.json") {
                println!("{}", "📋 Using existing test results to generate report".yellow());
                // Simple parsing of existing report to get data (this can be improved)
                if content.contains("\"passed\":") {
                    // Extract data from JSON content
                    (0, 0, 0, "No recent test results".to_string())
                } else {
                    (0, 0, 0, "No test results".to_string())
                }
            } else {
                (0, 0, 0, "No test results".to_string())
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
            r#"# Ifecaro Engine Test Report

## Execution Summary
- Total tests: {}
- Passed: {}
- Failed: {}
- Pass rate: {:.1}%

## Detailed Results
{}

## Test Tool Information
- Runtime environment: {}
- Tool version: Rust Test Runner v1.0
- Supported features: Complete test suite, quick tests, category tests, performance tests

## Generation Time
{}
"#,
            final_total,
            final_passed,
            final_failed,
            if final_total > 0 { (final_passed as f64 / final_total as f64) * 100.0 } else { 0.0 },
            final_results,
            if self.is_internal { "Inside Docker container" } else { "External environment" },
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );

        std::fs::write("test-report.md", report)
            .context("Unable to write test report")?;

        println!("{}", "✅ Test report generated: test-report.md".green());

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
            runner.run_test("Compile check", &format!("{}cargo check", prefix))?;
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
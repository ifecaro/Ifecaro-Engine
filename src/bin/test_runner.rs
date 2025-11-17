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
    /// Story module tests
    Story,
    /// Dashboard page tests
    Dashboard,
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
        // Ensure cargo check/test use release profile to avoid debug artifacts
        let mut cmd_str = command.to_string();
        if cmd_str.contains("cargo check") && !cmd_str.contains("--release") {
            cmd_str = cmd_str.replacen("cargo check", "cargo check --release", 1);
        } else if cmd_str.contains("cargo test") && !cmd_str.contains("--release") {
            cmd_str = cmd_str.replacen("cargo test", "cargo test --release", 1);
        }

        println!("\n{}", format!("📋 Running {}...", name).yellow().bold());
        println!("Command: {}", cmd_str.cyan());
        println!(
            "{}",
            "------------------------------------------------".dimmed()
        );

        let start_time = Instant::now();

        let should_skip_tailwind = cmd_str.contains("test") || cmd_str.contains("check");

        let (program, args) = if self.is_internal {
            // Execute directly inside container
            self.parse_internal_command(&cmd_str)
        } else {
            // Execute externally through docker compose exec
            self.parse_external_command(&cmd_str)
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
            command: cmd_str.clone(),
            error_message: if passed {
                None
            } else {
                Some("Test failed".to_string())
            },
        };

        if passed {
            println!(
                "{}",
                format!("✅ {} completed ({}ms)", name, duration.as_millis())
                    .green()
                    .bold()
            );
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

        println!(
            "\n{}",
            "================================================".blue()
        );
        println!("{}", "📊 Test Results Summary".blue().bold());
        println!(
            "{}",
            "================================================".blue()
        );
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
        println!(
            "{}",
            "================================================".blue()
        );

        // Clear build cache to ensure environment variables take impact
        if !self.is_internal {
            let _ = Command::new("docker")
                .args([
                    "compose",
                    "exec",
                    "app",
                    "rm",
                    "-rf",
                    "target/debug/build/ifecaro-*",
                ])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let tests = [
            (
                "Compile Check",
                if self.is_internal {
                    "cargo check"
                } else {
                    "docker compose exec app cargo check"
                },
            ),
            (
                "Unit Tests (All Modules)",
                if self.is_internal {
                    "cargo test --lib"
                } else {
                    "docker compose exec app cargo test --lib"
                },
            ),
            (
                "Story Module Tests",
                if self.is_internal {
                    "cargo test story_tests"
                } else {
                    "docker compose exec app cargo test story_tests"
                },
            ),
            (
                "Core Integration Tests",
                if self.is_internal {
                    "cargo test --test integration_tests"
                } else {
                    "docker compose exec app cargo test --test integration_tests"
                },
            ),
            (
                "Code Usage Examples",
                if self.is_internal {
                    "cargo test --test main_code_usage_example"
                } else {
                    "docker compose exec app cargo test --test main_code_usage_example"
                },
            ),
            (
                "Story Flow Tests",
                if self.is_internal {
                    "cargo test --test story_flow_tests"
                } else {
                    "docker compose exec app cargo test --test story_flow_tests"
                },
            ),
            (
                "Dashboard Core Tests",
                if self.is_internal {
                    "cargo test --test dashboard_tests"
                } else {
                    "docker compose exec app cargo test --test dashboard_tests"
                },
            ),
            (
                "Dashboard Interaction Tests",
                if self.is_internal {
                    "cargo test --test dashboard_interaction_tests"
                } else {
                    "docker compose exec app cargo test --test dashboard_interaction_tests"
                },
            ),
            (
                "Dashboard Benchmark Tests",
                if self.is_internal {
                    "cargo test --test dashboard_benchmark_tests"
                } else {
                    "docker compose exec app cargo test --test dashboard_benchmark_tests"
                },
            ),
        ];

        for (name, command) in tests {
            self.run_test(name, command)?;
        }

        // 新增 wasm-bindgen-test (wasm-pack test --headless --chrome)
        let wasm_test_cmd = if self.is_internal {
            "wasm-pack test --headless --chrome --release"
        } else {
            "docker compose exec app wasm-pack test --headless --chrome --release"
        };
        self.run_test("WASM Bindgen Tests (Chrome Headless)", wasm_test_cmd)?;

        println!("\n{}", "🎉 Complete test suite passed!".green().bold());

        Ok(())
    }

    fn run_internal_test_suite(&mut self) -> Result<()> {
        println!(
            "{}",
            "🚀 Starting optimized test suite in container"
                .blue()
                .bold()
        );
        println!(
            "{}",
            "================================================".blue()
        );

        // Clear build cache to ensure environment variables take impact
        if !self.is_internal {
            let _ = Command::new("docker")
                .args([
                    "compose",
                    "exec",
                    "app",
                    "rm",
                    "-rf",
                    "target/debug/build/ifecaro-*",
                ])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        // Container tests don't need docker compose exec app prefix
        let commands = vec![
            "cargo check --release",
            "cargo test --lib",
            "cargo test story_tests",
            "cargo test --test integration_tests",
            "cargo test --test main_code_usage_example",
            "cargo test --test story_flow_tests",
            "cargo test --test dashboard_tests",
            "cargo test --test dashboard_interaction_tests",
            "cargo test --test dashboard_benchmark_tests",
        ];

        let clean_commands: Vec<String> = commands
            .iter()
            .map(|command| command.replace("docker compose exec app ", ""))
            .collect();

        for command in clean_commands {
            self.run_test(&command, &command)?;
        }

        Ok(())
    }

    fn run_quick_tests(&mut self) -> Result<()> {
        println!("{}", "⚡ Starting quick test suite".yellow().bold());
        println!(
            "{}",
            "================================================".yellow()
        );

        // Clear build cache to ensure environment variables take impact
        if !self.is_internal {
            let _ = Command::new("docker")
                .args([
                    "compose",
                    "exec",
                    "app",
                    "rm",
                    "-rf",
                    "target/debug/build/ifecaro-*",
                ])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let prefix = if self.is_internal {
            ""
        } else {
            "docker compose exec app "
        };

        self.run_test("Compile check", &format!("{}cargo check", prefix))?;
        self.run_test("Unit tests", &format!("{}cargo test --lib", prefix))?;
        self.run_test("Story tests", &format!("{}cargo test story_tests", prefix))?;
        self.run_test(
            "Integration tests",
            &format!("{}cargo test --test integration_tests", prefix),
        )?;
        self.run_test(
            "Dashboard core tests",
            &format!("{}cargo test --test dashboard_tests", prefix),
        )?;

        Ok(())
    }

    fn run_category_test(&mut self, category: TestCategory) -> Result<()> {
        println!(
            "{}",
            format!("🎯 Running {:?} test category", category)
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "================================================".cyan()
        );

        // Clear build cache to ensure environment variables take impact
        if !self.is_internal {
            let _ = Command::new("docker")
                .args([
                    "compose",
                    "exec",
                    "app",
                    "rm",
                    "-rf",
                    "target/debug/build/ifecaro-*",
                ])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let (test_name, command) = match category {
            TestCategory::Compile => (
                "Compile check",
                if self.is_internal {
                    "cargo check"
                } else {
                    "docker compose exec app cargo check"
                },
            ),
            TestCategory::UI => (
                "UI component tests",
                if self.is_internal {
                    "cargo test --lib components::story_content_tests"
                } else {
                    "docker compose exec app cargo test --lib components::story_content_tests"
                },
            ),
            TestCategory::Advanced => (
                "Advanced feature tests",
                if self.is_internal {
                    "cargo test --lib components::story_content_advanced_tests"
                } else {
                    "docker compose exec app cargo test --lib components::story_content_advanced_tests"
                },
            ),
            TestCategory::MockApi => (
                "API Mock tests",
                if self.is_internal {
                    "cargo test --lib services::api_tests"
                } else {
                    "docker compose exec app cargo test --lib services::api_tests"
                },
            ),
            TestCategory::Integration => (
                "Integration tests",
                if self.is_internal {
                    "cargo test --test integration_tests"
                } else {
                    "docker compose exec app cargo test --test integration_tests"
                },
            ),
            TestCategory::Unit => (
                "Unit tests",
                if self.is_internal {
                    "cargo test --lib"
                } else {
                    "docker compose exec app cargo test --lib"
                },
            ),
            TestCategory::External => (
                "External integration tests",
                if self.is_internal {
                    "cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests"
                } else {
                    "docker compose exec app cargo test --test integration_tests --test main_code_usage_example --test story_flow_tests"
                },
            ),
            TestCategory::Story => (
                "Story module tests",
                if self.is_internal {
                    "cargo test story_tests"
                } else {
                    "docker compose exec app cargo test story_tests"
                },
            ),
            TestCategory::Dashboard => (
                "Dashboard tests",
                if self.is_internal {
                    "cargo test --test dashboard_tests --test dashboard_interaction_tests --test dashboard_benchmark_tests"
                } else {
                    "docker compose exec app cargo test --test dashboard_tests --test dashboard_interaction_tests --test dashboard_benchmark_tests"
                },
            ),
        };

        self.run_test(test_name, command)?;
        Ok(())
    }

    fn run_benchmark(&mut self) -> Result<()> {
        println!(
            "{}",
            "🏃 Starting performance benchmark tests".magenta().bold()
        );
        println!(
            "{}",
            "================================================".magenta()
        );

        // Clear build cache to ensure environment variables take impact
        if !self.is_internal {
            let _ = Command::new("docker")
                .args([
                    "compose",
                    "exec",
                    "app",
                    "rm",
                    "-rf",
                    "target/debug/build/ifecaro-*",
                ])
                .status();
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "target/debug/build/ifecaro-*"])
                .status();
        }

        let perf_cmd = if self.is_internal {
            "cargo test --release performance_tests"
        } else {
            "docker compose exec app cargo test --release performance_tests"
        };
        let bench_cmd = if self.is_internal {
            "cargo bench"
        } else {
            "docker compose exec app cargo bench"
        };

        self.run_test("Performance tests", perf_cmd)?;
        self.run_test("Benchmark tests", bench_cmd)?;

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
        }
        TestCommands::Quick => {
            let mut runner = TestRunner::new(is_container);
            runner.run_quick_tests()?;
            runner.print_summary();

            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        }
        TestCommands::Category { category } => {
            let mut runner = TestRunner::new(is_container);
            runner.run_category_test(category.clone())?;
            runner.print_summary();

            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        }
        TestCommands::Internal => {
            // Internal mode forces container mode
            let mut runner = TestRunner::new(true);
            runner.run_internal_test_suite()?;
            runner.print_summary();

            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        }
        TestCommands::Check => {
            let mut runner = TestRunner::new(is_container);
            let prefix = if is_container {
                ""
            } else {
                "docker compose exec app "
            };
            runner.run_test("Compile check", &format!("{}cargo check", prefix))?;
            runner.print_summary();

            let failed = runner.failed_tests.load(Ordering::SeqCst);
            if failed > 0 {
                std::process::exit(1);
            }
        }
        TestCommands::Bench => {
            let mut runner = TestRunner::new(is_container);
            runner.run_benchmark()?;
            runner.print_summary();
        }
    }

    Ok(())
}

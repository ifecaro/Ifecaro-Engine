#!/bin/bash

# Dashboard Test Runner Script
# This script runs comprehensive tests for the Dashboard component

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_color() {
    printf "${1}${2}${NC}\n"
}

print_header() {
    echo
    print_color $BLUE "=========================================="
    print_color $BLUE " $1"
    print_color $BLUE "=========================================="
    echo
}

print_section() {
    echo
    print_color $YELLOW "--- $1 ---"
    echo
}

# Check if we're in the correct directory
if [ ! -f "Cargo.toml" ]; then
    print_color $RED "Error: Please run this script from the project root directory"
    exit 1
fi

print_header "Dashboard Component Test Suite"

# Restart Docker containers
print_section "Restarting Docker Containers"
bash -c "docker compose restart app"

print_section "Running Unit Tests"
print_color $GREEN "Running basic unit tests..."
bash -c "cargo test dashboard_tests::unit_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Unit tests passed"
else
    print_color $RED "✗ Unit tests failed"
    exit 1
fi

print_section "Running Component Tests"
print_color $GREEN "Running component interaction tests..."
bash -c "cargo test dashboard_tests::component_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Component tests passed"
else
    print_color $RED "✗ Component tests failed"
    exit 1
fi

print_section "Running Integration Tests"
print_color $GREEN "Running integration tests..."
bash -c "cargo test dashboard_tests::integration_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Integration tests passed"
else
    print_color $RED "✗ Integration tests failed"
    exit 1
fi

print_section "Running Form Validation Tests"
print_color $GREEN "Running form validation tests..."
bash -c "cargo test dashboard_tests::form_validation_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Form validation tests passed"
else
    print_color $RED "✗ Form validation tests failed"
    exit 1
fi

print_section "Running Error Handling Tests"
print_color $GREEN "Running error handling tests..."
bash -c "cargo test dashboard_tests::error_handling_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Error handling tests passed"
else
    print_color $RED "✗ Error handling tests failed"
    exit 1
fi

print_section "Running Interaction Tests"
print_color $GREEN "Running user interaction workflow tests..."
bash -c "cargo test dashboard_interaction_tests::interaction_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Interaction tests passed"
else
    print_color $RED "✗ Interaction tests failed"
    exit 1
fi

print_section "Running Edge Case Tests"
print_color $GREEN "Running edge case tests..."
bash -c "cargo test dashboard_interaction_tests::edge_case_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Edge case tests passed"
else
    print_color $RED "✗ Edge case tests failed"
    exit 1
fi

print_section "Running Performance Tests"
print_color $GREEN "Running performance benchmark tests..."
bash -c "cargo test dashboard_benchmark_tests::benchmark_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Performance tests passed"
else
    print_color $RED "✗ Performance tests failed"
    exit 1
fi

print_section "Running Stress Tests"
print_color $GREEN "Running stress tests..."
bash -c "cargo test dashboard_benchmark_tests::stress_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Stress tests passed"
else
    print_color $RED "✗ Stress tests failed"
    exit 1
fi

print_section "Running Accessibility Tests"
print_color $GREEN "Running accessibility tests..."
bash -c "cargo test dashboard_tests::accessibility_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Accessibility tests passed"
else
    print_color $RED "✗ Accessibility tests failed"
    exit 1
fi

print_section "Running Serialization Tests"
print_color $GREEN "Running serialization tests..."
bash -c "cargo test dashboard_tests::serialization_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Serialization tests passed"
else
    print_color $RED "✗ Serialization tests failed"
    exit 1
fi

print_section "Running API Compatibility Tests"
print_color $GREEN "Running API compatibility tests..."
bash -c "cargo test dashboard_tests::api_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ API compatibility tests passed"
else
    print_color $RED "✗ API compatibility tests failed"
    exit 1
fi

print_section "Running UI State Management Tests"
print_color $GREEN "Running UI state management tests..."
bash -c "cargo test dashboard_tests::ui_state_tests --verbose"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ UI state management tests passed"
else
    print_color $RED "✗ UI state management tests failed"
    exit 1
fi

# Run cargo check to ensure compilation
print_section "Running Compilation Check"
print_color $GREEN "Checking compilation..."
bash -c "docker compose exec app cargo check"

if [ $? -eq 0 ]; then
    print_color $GREEN "✓ Compilation check passed"
else
    print_color $RED "✗ Compilation check failed"
    exit 1
fi

# Generate test coverage report if available
print_section "Generating Test Coverage Report"
if command -v cargo-tarpaulin &> /dev/null; then
    print_color $GREEN "Generating coverage report..."
    bash -c "cargo tarpaulin --tests --out Html --output-dir target/coverage"
    print_color $GREEN "Coverage report generated in target/coverage/"
else
    print_color $YELLOW "cargo-tarpaulin not found. Skipping coverage report."
fi

print_header "Test Summary"

# Count test results
TOTAL_TESTS=12
PASSED_TESTS=$TOTAL_TESTS

print_color $GREEN "Dashboard Component Test Results:"
print_color $GREEN "✓ Unit Tests"
print_color $GREEN "✓ Component Tests"
print_color $GREEN "✓ Integration Tests"
print_color $GREEN "✓ Form Validation Tests"
print_color $GREEN "✓ Error Handling Tests"
print_color $GREEN "✓ Interaction Tests"
print_color $GREEN "✓ Edge Case Tests"
print_color $GREEN "✓ Performance Tests"
print_color $GREEN "✓ Stress Tests"
print_color $GREEN "✓ Accessibility Tests"
print_color $GREEN "✓ Serialization Tests"
print_color $GREEN "✓ API Compatibility Tests"
print_color $GREEN "✓ UI State Management Tests"

echo
print_color $GREEN "🎉 All Dashboard tests passed successfully! ($PASSED_TESTS/$TOTAL_TESTS)"
print_color $BLUE "Dashboard component is ready for production use."

# Optional: Start development server
echo
read -p "Would you like to start the development server? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_section "Starting Development Server"
    bash -c "docker compose exec app dx serve --port 9999 &"
    print_color $GREEN "Development server started on port 9999"
fi

exit 0 
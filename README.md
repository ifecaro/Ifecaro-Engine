# Ifecaro Engine 🚀

A modern web-based interactive story engine built with Rust and Dioxus, featuring comprehensive testing suite, automated deployment tools, and an intuitive command-line interface.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Quick Start](#quick-start)
- [Development Tools](#development-tools)
- [Interactive Menu](#interactive-menu)
- [Deployment](#deployment)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

## 🎯 Overview

Ifecaro Engine is a sophisticated web application for creating and managing interactive stories. Built with:

- **Frontend**: Dioxus (Rust-based web framework)
- **Styling**: Tailwind CSS (compiled via build.rs)
- **Architecture**: PWA-ready with Service Worker support
- **Testing**: Comprehensive test suite with 75+ unit tests
- **Deployment**: Automated Rust-based deployment pipeline with interactive CLI

## ✨ Features

### Core Features
- 📚 Interactive story content management
- 🎮 Dynamic choice system with countdown timers
- 🌐 Multi-language support (i18n)
- 📱 Responsive design with mobile optimization
- ♿ Accessibility-first approach
- 🔄 Real-time state management
- 💾 IndexedDB integration for offline support

### Development Features
- 🦀 **Rust-powered deployment tools with interactive menu**
- 🖥️ **Beautiful, user-friendly command-line interface**
- 🧪 Comprehensive testing (75+ unit tests, 13+ integration tests)
- 🎨 Automated Tailwind CSS compilation
- 🐳 Docker-based development environment
- 📦 PWA resource bundling
- 🚀 Automated remote deployment with SSH

## 🚀 Quick Start

### Prerequisites

- Docker and Docker Compose
- Git

### Using Docker (Recommended)

1. Start the development environment:
```bash
docker compose up -d
```

2. Launch the interactive deployment menu:
```bash
docker compose exec app cargo run --bin deploy
```

This will show you a beautiful, easy-to-use menu:

```
🚀 Ifecaro 引擎部署工具
================================================

請選擇要執行的操作:

  1.  📋 快速檢查 (cargo check)
  2.  🧪 執行測試套件
  3.  🏗  構建專案
  4.  🧹 清理構建檔案
  5.  ⚡ 開發模式 (檢查 + 快速測試)
  6.  🎯 生產模式 (完整一鍵部署)
  0.  ❌ 退出

請輸入選項 (0-6):
```

3. Or use direct commands for automation:
```bash
# Test commands (using Rust test runner)
cargo run --bin test-runner full      # Complete test suite
cargo run --bin test-runner quick     # Quick tests (compile + basic UI + API mock)
cargo run --bin test-runner internal  # Container-optimized testing
cargo run --bin test-runner category compile  # Specific test category
cargo run --bin test-runner benchmark # Performance benchmarks
cargo run --bin test-runner report    # Generate test report

# Deployment commands (using Rust deployment tool)
cargo run --bin deploy check      # Quick check (cargo check)
cargo run --bin deploy test full  # Run complete test suite
cargo run --bin deploy build      # Build project
cargo run --bin deploy deploy     # Full deployment pipeline
cargo run --bin deploy dev        # Development mode (check + quick test)
cargo run --bin deploy prod       # Production mode (full test + build + deploy)
```

## 🛠️ Development Tools

### Rust Deployment CLI

The project includes a powerful Rust-based CLI tool for managing development workflows with both interactive and command-line modes:

```bash
# Interactive mode - Beautiful menu interface
docker compose exec app cargo run --bin deploy

# Command-line mode - Direct automation
docker compose exec app cargo run --bin deploy <command>
```

#### Available Commands

| Command | Description | Execution Details | Use Case | Duration |
|---------|-------------|-------------------|----------|----------|
| `check` | Quick cargo check | Compilation verification only | Quick syntax check | ~5s |
| `test` | Run test suite with submenu | Interactive test mode selection | Test-specific scenarios | 15s-60s |
| `build` | Build Rust + Dioxus project | cargo build + dx build | Build verification | ~90s |
| `clean` | Clean build artifacts | Remove target/ and dx/ directories | Cleanup, fresh start | ~5s |
| `dev` | Development mode | check + quick test | **Daily development** | ~20s |
| `prod` | Production mode | full test + build + deploy + remote | **Production deployment** | ~90s |

#### Menu Options Detailed Comparison

| Option | Name | Execution Flow | Steps | Output | Best For |
|--------|------|----------------|-------|---------|----------|
| **1** | 📋 快速檢查 | `cargo check` | 1. Compilation check | ✅ Syntax verification | Quick verification |
| **2** | 🧪 執行測試套件 | Opens test submenu | 1. Test mode selection<br>2. Test execution | ✅ Test results | Specific testing needs |
| **3** | 🏗 構建專案 | `build()` | 1. Rust release build<br>2. Dioxus web build | ✅ Built artifacts | Build verification |
| **4** | 🧹 清理構建檔案 | `clean()` | 1. Remove target/<br>2. Remove dx/ | ✅ Clean workspace | Fresh start, disk space |
| **5** | ⚡ 開發模式 | `check() + test(quick)` | 1. Cargo check<br>2. Quick test suite | ✅ Development ready | **Daily development** |
| **6** | 🎯 生產模式 | `deploy()` | 1. Full test suite<br>2. Rust + Dioxus build<br>3. PWA bundling<br>4. Deploy package<br>5. Remote upload<br>6. Service restart | ✅ Production deployed | **Production deployment** |

#### Performance Comparison

| Mode | Time | Test Coverage | Build | Deploy | SSH Upload | Remote Restart |
|------|------|---------------|-------|--------|------------|----------------|
| **Quick Check** | ~5s | ❌ None | ❌ No | ❌ No | ❌ No | ❌ No |
| **Test Suite** | 15-60s | ✅ Configurable | ❌ No | ❌ No | ❌ No | ❌ No |
| **Build** | ~90s | ❌ None | ✅ Yes | ❌ No | ❌ No | ❌ No |
| **Clean** | ~5s | ❌ None | ❌ Removes | ❌ No | ❌ No | ❌ No |
| **Development** | ~20s | ✅ Quick tests | ❌ No | ❌ No | ❌ No | ❌ No |
| **Production** | ~90s | ✅ Full tests | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |

#### Recommended Workflows

##### 🔄 Daily Development Workflow
```bash
# Start development environment
docker compose up -d

# Launch interactive menu
docker compose exec app cargo run --bin deploy

# Select option 5: ⚡ 開發模式 (檢查 + 快速測試)
# This runs: cargo check + quick test suite (~20s)
```

##### 🚀 Production Deployment Workflow
```bash
# For production deployment
docker compose exec app cargo run --bin deploy

# Select option 6: 🎯 生產模式 (完整一鍵部署)
# This runs: Full test → Build → Package → Upload → Restart (~90s)
```

##### 🐛 Debugging Workflow
```bash
# For testing specific issues
docker compose exec app cargo run --bin deploy

# Select option 2: 🧪 執行測試套件
# Then choose specific test category (Full/Quick/Internal)
```

##### 🧹 Maintenance Workflow
```bash
# Clean up before fresh start
docker compose exec app cargo run --bin deploy

# Select option 4: 🧹 清理構建檔案
# Then run development or production mode
```

#### Command Line Automation

For automated scripts and CI/CD:

```bash
# Direct command execution (no interactive menu)
docker compose exec app cargo run --bin deploy dev    # Development check
docker compose exec app cargo run --bin deploy prod   # Production deployment
docker compose exec app cargo run --bin deploy check  # Quick verification
docker compose exec app cargo run --bin deploy clean  # Cleanup
```

## 🖥️ Interactive Menu

### Main Menu Features

- **🎨 Beautiful Visual Design**: Clean, colorful interface with emoji icons
- **📝 Clear Options**: Numbered menu items with descriptive text
- **🔄 Easy Navigation**: Simple number input system
- **⚡ Quick Actions**: Direct access to common development tasks
- **🛡️ Error Handling**: Invalid input protection and user feedback

### Test Submenu

When selecting "執行測試套件" (option 2), you get a specialized test menu:

```
🧪 測試套件選單
================================================

請選擇測試模式:

  1.  🎯 完整測試套件 (所有測試)
  2.  ⚡ 快速測試 (編譯 + 基礎測試)
  3.  🐳 容器內優化測試
  0.  ↩️ 返回主選單

請輸入選項 (0-3):
```

#### Test Mode Comparison

| Mode | Description | Test Coverage | Duration | Use Case |
|------|-------------|---------------|----------|----------|
| **完整測試套件** | All 75+ tests | Unit + Integration + E2E | ~60s | Full verification |
| **快速測試** | Essential tests only | Basic UI + API mock | ~15s | Quick verification |
| **容器內優化測試** | Docker optimized | Full suite optimized | ~45s | Container environment |

#### 🚀 Efficiency Optimization

The menu has been optimized to eliminate redundant operations:

**Previous Issue:**
- "完整部署流程" and "生產模式" performed duplicate operations
- Tests were run twice, builds were executed twice
- Total time: ~180 seconds with redundancy

**Current Optimization:**
- Single **🎯 生產模式** provides optimized one-click deployment
- No duplicate test or build steps
- Total time: ~90 seconds (50% improvement)

**Why Each Option Matters:**
- **Options 1-4**: Individual operations for specific needs
- **Option 5 (Development)**: Fast daily workflow (check + quick test)
- **Option 6 (Production)**: Complete optimized deployment pipeline

### Menu Benefits

- **👨‍💻 Developer-Friendly**: No need to remember complex commands
- **🚀 Faster Workflow**: Quick access to common operations
- **📱 Consistent UX**: Same interface across all development tasks
- **🎯 Reduced Errors**: Guided menu prevents command typos
- **📚 Self-Documenting**: Each option clearly explains what it does

#### Examples

```bash
# Interactive development workflow (recommended for new users)
docker compose exec app cargo run --bin deploy
# Then select: 5 (Development mode) - Fast daily workflow

# Interactive production deployment
docker compose exec app cargo run --bin deploy  
# Then select: 6 (Production mode) - Complete one-click deployment

# Interactive testing with options
docker compose exec app cargo run --bin deploy
# Then select: 2 (Test suite) → 2 (Quick test) for fast verification

# Command-line production deployment (for CI/CD)
docker compose exec app cargo run --bin deploy prod

# Quick verification without menu
docker compose exec app cargo run --bin deploy check

# Clean workspace before fresh start
docker compose exec app cargo run --bin deploy clean
```

## 🚀 Deployment

### Configuration

Create a `.env` file with deployment configuration:

```env
DEPLOY_USER=your-username
DEPLOY_HOST=your-server-ip
DEPLOY_PATH=/path/to/deployment
SSH_KEY_PATH=/root/.ssh
```

### Deployment Pipeline

The automated deployment process includes:

1. **Testing Phase**
   - ✅ Compilation checks
   - ✅ 75+ unit tests
   - ✅ Integration tests
   - ✅ API mock tests

2. **Build Phase**
   - 🏗️ Rust release build
   - 🎯 Dioxus web compilation
   - 🎨 Tailwind CSS processing
   - 📦 PWA resource bundling

3. **Deployment Phase**
   - 📤 Secure SCP upload
   - 📁 Remote extraction to `frontend/` directory
   - 🔄 Docker service restart
   - ✅ Deployment verification

### Manual Deployment

```bash
# Full production deployment
docker compose exec app cargo run --bin deploy prod

# Deploy only (skip tests/build)
docker compose exec app cargo run --bin deploy deploy
```

## 🧪 Testing

### Test Architecture

The comprehensive test suite consists of multiple test files covering everything from basic UI components to complex API integrations:

```
src/
├── components/
│   ├── story_content_tests.rs              # Basic UI Tests (27 tests)
│   ├── story_content_advanced_tests.rs     # Advanced Feature Tests (19 tests)
│   └── story_content_api_integration_tests.rs # API Integration Tests (6 tests)
└── services/
    └── api_tests.rs                         # API Mock Tests (10 tests)
```

### Test Coverage Summary

#### Functional Coverage
- ✅ **Basic UI Rendering**: Text display, choice lists, chapter titles
- ✅ **Interactive Features**: Choice enable/disable, countdown timers, keyboard navigation  
- ✅ **Responsive Design**: Multiple screen sizes, dark mode support
- ✅ **Accessibility Features**: Semantic tags, focus management, WCAG compliance
- ✅ **Data Processing**: JSON serialization, multilingual support
- ✅ **API Integration**: Mock testing, error handling, data flow
- ✅ **Edge Cases**: Empty data, extremely long content, special characters
- ✅ **Performance Testing**: Large dataset rendering, memory optimization
- ✅ **Regression Testing**: Protection against known issues

#### Test Type Distribution
- **Unit Tests**: 54 tests (85%) - Component logic, UI rendering, state management
- **Integration Tests**: 6 tests (9%) - End-to-end workflows, API integration  
- **End-to-End Tests**: 4 tests (6%) - Complete user scenarios

### Detailed Test Categories

| Category | Tests | File | Description |
|----------|-------|------|-------------|
| **Basic UI Tests** | 3 | `story_content_tests.rs` | Core component rendering |
| **Choice System** | 6 | `story_content_tests.rs` | Interactive choice mechanics |
| **Responsive Design** | 3 | `story_content_tests.rs` | Mobile/desktop layouts |
| **Accessibility** | 3 | `story_content_tests.rs` | WCAG compliance, screen readers |
| **Edge Cases** | 6 | `story_content_tests.rs` | Error handling, Unicode, special chars |
| **Integration Style** | 2 | `story_content_tests.rs` | CSS classes, complete UI structure |
| **Performance** | 2 | `story_content_tests.rs` | Large datasets, complex structures |
| **Regression** | 3 | `story_content_tests.rs` | Bug prevention, known issue fixes |
| **Choice Data Structure** | 3 | `story_content_advanced_tests.rs` | Choice object creation, serialization |
| **Action Type Validation** | 4 | `story_content_advanced_tests.rs` | goto, set, add, custom actions |
| **Choice Array Operations** | 4 | `story_content_advanced_tests.rs` | Array handling, filtering |
| **Enabled Choice Logic** | 3 | `story_content_advanced_tests.rs` | Matching logic, countdown impact |
| **Countdown State** | 3 | `story_content_advanced_tests.rs` | Timer initialization, expiration |
| **Keyboard Input** | 3 | `story_content_advanced_tests.rs` | Number parsing, index calculation |
| **API Mock Functions** | 6 | `api_tests.rs` | CRUD operations, success/failure |
| **Complex API Data** | 2 | `api_tests.rs` | Multilingual content, complex JSON |
| **Multilingual Support** | 2 | `api_tests.rs` | Chinese, English, Japanese content |
| **End-to-End Integration** | 2 | `story_content_api_integration_tests.rs` | Complete data flow |
| **API Error Handling** | 2 | `story_content_api_integration_tests.rs` | Fallback mechanisms |
| **Complex Scenarios** | 2 | `story_content_api_integration_tests.rs` | Time limits, edge cases |

### Key Test Features

#### 1. Basic UI Tests (27 tests)
- **Empty Story Content**: Basic rendering with empty content
- **Paragraph Display**: Multi-paragraph Chinese text rendering
- **Chapter Title Styling**: Typography and responsive design
- **Choice States**: Single/multiple choices, enabled/disabled states
- **Responsive Classes**: `sm:`, `md:`, `lg:` breakpoints
- **Dark Mode**: `dark:` prefixed styles
- **Accessibility**: Semantic `<ol>`/`<li>` tags, focus states
- **Edge Cases**: Long content (500+ chars), Unicode/emoji, special characters

#### 2. Advanced Feature Tests (19 tests)
- **Choice Data Structures**: Object creation, JSON serialization
- **Action Types**: goto, set, add, custom action validation
- **Array Operations**: Filtering, iteration, empty array handling
- **Countdown Logic**: Timer setup, expiration handling
- **Keyboard Navigation**: Number key parsing, index calculation

#### 3. API Integration Tests (16 tests)
- **Mock API**: CRUD operations, success/failure scenarios
- **Data Conversion**: API data to UI component integration
- **Error Handling**: Fallback mechanisms, error display
- **Multilingual**: Chinese/English/Japanese content support
- **Complex Scenarios**: Time limits, incomplete data handling

### Test Execution Methods

```bash
# Using Rust test runner (recommended)
cargo run --bin test-runner full      # Complete test suite
cargo run --bin test-runner quick     # Quick tests (compile + basic UI + API mock) 
cargo run --bin test-runner internal  # Container-optimized testing
cargo run --bin test-runner category compile  # Specific test category
cargo run --bin test-runner benchmark # Performance benchmarks
cargo run --bin test-runner report    # Generate test report

# Using deployment tool
cargo run --bin deploy test full      # Run complete test suite
cargo run --bin deploy dev            # Development mode (check + quick test)

# Direct cargo commands
cargo test story_content_tests
cargo test story_content_advanced_tests  
cargo test api_tests
cargo test integration_tests
```

### Code Quality Assurance

- ✅ **Compilation Check**: Zero warnings, zero errors
- ✅ **Type Safety**: Guaranteed by Rust type system  
- ✅ **Memory Safety**: No memory leak risks
- ✅ **Concurrency Safety**: Appropriate synchronization mechanisms
- ✅ **Performance Monitoring**: Test execution time tracking
- ✅ **Coverage Analysis**: Comprehensive functional coverage

### Continuous Integration

- **Pre-commit**: Automatically execute complete test suite
- **Pre-build**: Run all tests before production compilation
- **Docker Environment**: Ensures consistent test environment
- **Automated Deployment**: Tests must pass before deployment

## 📁 Project Structure

```
Ifecaro-Engine/
├── src/
│   ├── bin/
│   │   ├── deploy.rs           # Interactive Rust deployment CLI
│   │   └── test_runner.rs      # Comprehensive Rust test runner CLI
│   ├── components/             # Dioxus components
│   │   ├── story_content.rs    # Main story component
│   │   ├── story_content_tests.rs              # Basic UI Tests (27 tests)
│   │   ├── story_content_advanced_tests.rs     # Advanced Feature Tests (19 tests)
│   │   └── story_content_api_integration_tests.rs # API Integration Tests (6 tests)
│   ├── contexts/               # State management
│   ├── pages/                  # Page components
│   ├── services/               # API services
│   │   └── api_tests.rs        # API Mock Tests (10 tests)
│   └── main.rs                 # Application entry
├── tests/                      # External integration tests
├── docs/                       # Documentation
│   └── Test-Documentation.md   # Comprehensive test documentation
├── public/                     # Static assets
│   ├── manifest.json           # PWA manifest
│   ├── sw.js                   # Service worker
│   └── img/icons/              # App icons
├── build.rs                    # Build script (Tailwind CSS)
├── docker-compose.yml          # Development environment
├── Dockerfile                  # Container configuration
├── tailwind.config.js          # Tailwind configuration
└── .env                        # Deployment configuration
```

## 🔧 Build System

### Tailwind CSS Integration

Automatic Tailwind CSS compilation via `build.rs`:

```rust
// Triggers on file changes:
// - src/input.css
// - tailwind.config.js  
// - src/ directory changes
```

### PWA Support

- 📱 Web App Manifest
- 🔄 Service Worker for offline support
- 🎨 App icons (multiple sizes)
- 📦 Optimized asset bundling

## 🤝 Contributing

### Development Workflow

1. **Setup development environment**
   ```bash
   docker compose up -d
   ```

2. **Use the interactive development menu (recommended)**
   ```bash
   docker compose exec app cargo run --bin deploy
   # Select option 6: ⚡ 開發模式 (檢查 + 快速測試)
   ```

3. **Or use direct commands for specific tasks**
   ```bash
   # Quick verification
   docker compose exec app cargo run --bin deploy check
   
   # Run tests interactively
   docker compose exec app cargo run --bin deploy
   # Then select: 2 (Test suite) → 2 (Quick test)
   
   # Command-line testing
   docker compose exec app cargo run --bin deploy test quick
   ```

4. **Build and verify**
```bash
   # Interactive build
   docker compose exec app cargo run --bin deploy
   # Select option 3: 🏗️ 構建專案
   
   # Or direct command
   docker compose exec app cargo run --bin deploy build
   ```

### Developer Experience Improvements

- **🖥️ Interactive Menu**: Beautiful, user-friendly CLI interface
- **🎯 Quick Access**: No need to memorize complex commands
- **📝 Clear Feedback**: Detailed status messages and progress indicators
- **🛡️ Error Prevention**: Menu validation prevents common mistakes
- **⚡ Streamlined Workflow**: Optimized for common development tasks

### Code Quality Requirements

- ✅ All tests must pass (75+ unit tests, 13+ integration tests)
- ✅ No compilation warnings
- ✅ Accessible UI components (WCAG compliance)
- ✅ Responsive design (mobile/desktop)
- ✅ Comprehensive error handling
- ✅ Type-safe Rust code
- ✅ Performance optimization

### Adding Tests

Tests are organized by category in multiple files:

```rust
// Basic UI Tests - src/components/story_content_tests.rs
#[cfg(test)]
mod basic_ui_tests {
    use super::*;

    #[test]
    fn test_your_ui_feature() {
        // Your UI test implementation
    }
}

// Advanced Features - src/components/story_content_advanced_tests.rs  
#[cfg(test)]
mod advanced_feature_tests {
    use super::*;

    #[test]
    fn test_your_advanced_feature() {
        // Your advanced feature test
    }
}

// API Integration - src/components/story_content_api_integration_tests.rs
#[cfg(test)]
mod api_integration_tests {
    use super::*;

    #[test]
    fn test_your_api_integration() {
        // Your API integration test
    }
}
```

### Test Maintenance Guidelines

- **New Features**: Corresponding tests required for all new features
- **Bug Fixes**: Add regression tests to prevent reoccurrence  
- **Performance**: Include performance benchmarks for critical paths
- **Coverage**: Maintain high test coverage across all components
- **Documentation**: Update test documentation for significant changes

## 📄 License

This project is licensed under [LICENSE](LICENSE).

## 🆘 Support

For issues and questions:

1. **Use the Interactive Menu**: Launch `cargo run --bin deploy` for guided development workflows
2. **Check Test Results**: Use the test submenu to isolate specific issues
3. **Review Error Messages**: The Rust CLI tool provides detailed error information
4. **Verify Environment**: Ensure Docker environment is properly set up
5. **Check Configuration**: Verify deployment configuration in `.env` file
6. **Run Specific Tests**: Use test categories to isolate problems
7. **Consult Documentation**: Review test documentation for detailed descriptions

### Quick Troubleshooting

```bash
# Start with the interactive menu for guided troubleshooting
docker compose exec app cargo run --bin deploy

# Quick health check
docker compose exec app cargo run --bin deploy check

# Run quick tests to verify basic functionality
docker compose exec app cargo run --bin deploy test quick

# Check specific test categories
docker compose exec app cargo run --bin test-runner category compile
```

## 📚 Additional Documentation

- **[Test Documentation](docs/Test-Documentation.md)**: Comprehensive test suite documentation with detailed descriptions of all 75+ tests
- **Test Categories**: Basic UI, Advanced Features, API Integration, Performance, Accessibility
- **Test Architecture**: Four main test files covering complete application functionality

---

**Built with ❤️ using Rust and Dioxus** 
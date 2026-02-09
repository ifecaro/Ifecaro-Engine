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
- **Testing**: Comprehensive test suite with 97+ unit tests
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
- 📖 **Advanced Reader Mode with Smart Auto-Expansion**
  - 🎯 Automatic random choice selection
  - 🔄 Continuous story path expansion until completion
  - 🚫 Infinite loop prevention with visited tracking
  - 📊 Statistical choice distribution validation
  - 🌐 Multi-language story path consistency
- 📝 **Dashboard page with comprehensive content management**
- 🔧 **Advanced paragraph editing and chapter management**
- ⚡ **Multi-language content creation and validation**
- 🎯 **Real-time form validation with dynamic button states**
- 🌍 **Comprehensive language switching in edit mode**

### Development Features
- 🦀 **Rust-powered deployment tools with interactive menu**
- 🖥️ **Beautiful, user-friendly command-line interface**
- 🧪 Comprehensive testing (25+ reader mode tests, 182+ unit tests, 28+ integration tests, 25+ performance tests)
- 🎨 Automated Tailwind CSS compilation
- 🐳 Docker-based development environment
- 📦 PWA resource bundling
- 🚀 Automated remote deployment with SSH
- 🎯 **Complete Dashboard testing suite with 59 specialized tests**
- 🔍 **Advanced UI/content language switching test coverage**
- ⚡ **Form validation and button state management testing**
- 📖 **Reader Mode testing with 20 unit tests + 6 integration tests**

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
🚀 Ifecaro Engine Deployment Tool
================================================

Please select an operation:

  1.  📋 Quick Check (cargo check)
  2.  🧪 Run Test Suite
  3.  🏗  Build Project
  4.  🧹 Clean Build Files
  5.  ⚡ Development Mode (check + quick test)
  6.  🎯 Production Mode (complete one-click deployment)
  7.  🌐 Remote VPS Deploy (GHCR pull + docker compose up)
  0.  ❌ Exit

Please enter option (0-7):
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
cargo run --manifest-path tools/deploy-remote/Cargo.toml --release  # Standalone remote deploy (minimal deps)
cargo run --bin deploy remote     # Wrapper: delegates to standalone remote deploy
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
| `remote` | Remote VPS deploy (wrapper) | run standalone `tools/deploy-remote` binary | **Deploy GHCR images** | ~10-30s |

#### Menu Options Detailed Comparison

| Option | Name | Execution Flow | Steps | Output | Best For |
|--------|------|----------------|-------|---------|----------|
| **1** | 📋 Quick Check | `cargo check` | 1. Compilation check | ✅ Syntax verification | Quick verification |
| **2** | 🧪 Run Test Suite | Opens test submenu | 1. Test mode selection<br>2. Test execution | ✅ Test results | Specific testing needs |
| **3** | 🏗 Build Project | `build()` | 1. Rust release build<br>2. Dioxus web build | ✅ Built artifacts | Build verification |
| **4** | 🧹 Clean Build Files | `clean()` | 1. Remove target/<br>2. Remove dx/ | ✅ Clean workspace | Fresh start, disk space |
| **5** | ⚡ Development Mode | `check() + test(quick)` | 1. Cargo check<br>2. Quick test suite | ✅ Development ready | **Daily development** |
| **6** | 🎯 Production Mode | `deploy()` | 1. Full test suite<br>2. Rust + Dioxus build<br>3. PWA bundling<br>4. Deploy package<br>5. Remote upload<br>6. Service restart | ✅ Production deployed | **Production deployment** |
| **7** | 🌐 Remote VPS Deploy | `run_remote_deploy_binary()` | 1. Run standalone deploy binary<br>2. GHCR image pull<br>3. docker compose up -d | ✅ Remote services running | **Fast remote refresh** |

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

# Select option 5: ⚡ Development Mode (check + quick test)
# This runs: cargo check + quick test suite (~20s)
```

##### 🚀 Production Deployment Workflow
```bash
# For production deployment
docker compose exec app cargo run --bin deploy

# Select option 6: 🎯 Production Mode (complete one-click deployment)
# This runs: Full test → Build → Package → Upload → Restart (~90s)
```

##### 🐛 Debugging Workflow
```bash
# For testing specific issues
docker compose exec app cargo run --bin deploy

# Select option 2: 🧪 Run Test Suite
# Then choose specific test category (Full/Quick/Internal)
```

##### 🧹 Maintenance Workflow
```bash
# Clean up before fresh start
docker compose exec app cargo run --bin deploy

# Select option 4: 🧹 Clean Build Files
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

When selecting "Run Test Suite" (option 2), you get a specialized test menu:

```
🧪 Test Suite Menu
================================================

Please select test mode:

  1.  🎯 Complete Test Suite (all tests)
  2.  ⚡ Quick Test (compile + basic tests)
  3.  🐳 Container Optimized Test
  0.  ↩️ Return to Main Menu

Please enter option (0-3):
```

#### Test Mode Comparison

| Mode | Description | Test Coverage | Duration | Use Case |
|------|-------------|---------------|----------|----------|
| **Complete Test Suite** | All 97+ tests | Unit + Integration + E2E | ~60s | Full verification |
| **Quick Test** | Essential tests only | Basic UI + API mock | ~15s | Quick verification |
| **Container Optimized Test** | Docker optimized | Full suite optimized | ~45s | Container environment |

#### 🚀 Efficiency Optimization

The menu has been optimized to eliminate redundant operations:

**Previous Issue:**
- "Complete Deployment Process" and "Production Mode" performed duplicate operations
- Tests were run twice, builds were executed twice
- Total time: ~180 seconds with redundancy

**Current Optimization:**
- Single **🎯 Production Mode** provides optimized one-click deployment
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
# Server connection settings
DEPLOY_USER=your-username        # Your SSH username on the server
DEPLOY_HOST=your-server-ip       # Server IP address or domain name
DEPLOY_PATH=/home/your-username/ifecaro  # Deployment directory on server
SSH_KEY_PATH=/home/your-local-username/.ssh  # SSH key directory (optional when SSH_KEY_FILE is set)
SSH_KEY_NAME=id_ed25519                    # SSH key filename in SSH_KEY_PATH (default: id_rsa)
# SSH_KEY_FILE=/home/your-local-username/.ssh/id_ed25519  # Full key path (overrides path + name)
# SSH_KNOWN_HOSTS_FILE=/home/your-local-username/.ssh/known_hosts  # Optional known_hosts file override

# Example:
# DEPLOY_USER=developer
# DEPLOY_HOST=192.168.1.100
# DEPLOY_PATH=/home/developer/ifecaro
# SSH_KEY_PATH=/home/user/.ssh
# SSH_KEY_NAME=id_ed25519
# SSH_KEY_FILE=/home/user/.ssh/id_ed25519
# DEPLOY_COMPOSE_FILE=docker-compose.deploy.yml
# GHCR_TAG=v1.2.3
# GHCR_TAG_FORMAT=v{version}
# DEPLOY_ENV=staging
```

Note: Make sure to:
1. Replace `your-username` with your actual server username
2. Replace `your-server-ip` with your server's IP address
3. Replace `your-local-username` with your local machine username
4. If needed, set `SSH_KEY_NAME` (for example `id_ed25519`) or `SSH_KEY_FILE` (full path)
5. Ensure the deployment path exists on the server
6. Verify SSH key permissions (600 for private key, 644 for public key)
7. Place `docker-compose.deploy.yml` in `DEPLOY_PATH` (or set `DEPLOY_COMPOSE_FILE` to match)

### 取得 VPS 的 SSH Key（建議流程）

以下流程避免包含任何敏感資訊，僅提供一般做法：

1. 在本機產生一組 SSH 金鑰（若已存在可略過）：
   ```bash
   ssh-keygen -t ed25519 -C "your-email"
   ```
2. 將 **公開金鑰**（如 `~/.ssh/id_ed25519.pub`）加入 VPS：
   - 透過雲端主機提供的控制台／Web 介面上傳公鑰，或
   - 使用 VPS 的緊急／主控台登入，將公鑰加入 `~/.ssh/authorized_keys`
3. 確認 `~/.ssh` 權限與檔案權限正確：
   ```bash
   chmod 700 ~/.ssh
   chmod 600 ~/.ssh/authorized_keys
   ```
4. 在本機用以下方式測試連線：
   ```bash
   ssh -i ~/.ssh/id_ed25519 your-username@your-server-ip
   ```

> 提醒：請勿將私鑰（例如 `id_ed25519`）分享或提交到版本控制。

### Remote Compose File (GHCR Deploy)

The standalone remote deploy program (`tools/deploy-remote`) runs `docker compose -f <file> pull` and `up -d` on the server.
`cargo run --bin deploy remote` now acts as a wrapper that forwards execution to this standalone program, reducing dependency loading and startup overhead for remote-only deployment.
Create a deployment-specific compose file at `DEPLOY_PATH`, for example:

```yaml
services:
  pocketbase:
    image: ghcr.io/muchobien/pocketbase:latest
    environment:
      ENCRYPTION: ${PB_ENCRYPTION_KEY}
    ports:
      - "8090:8090"
    volumes:
      - ./data:/pb_data
      - ./public:/pb_public
      - ./hooks:/pb_hooks

  nginx:
    image: ${FRONTEND_IMAGE:-ghcr.io/muchobien/ifecaro-engine:${GHCR_TAG:-latest}}
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./certs:/etc/nginx/certs:ro
      - ${NGINX_CONF_PATH:-./nginx.conf}:/etc/nginx/conf.d/default.conf:ro
```

Set `PB_ENCRYPTION_KEY` in the server-side `.env` file, and optionally set `NGINX_CONF_PATH` / `FRONTEND_IMAGE` to control nginx config and the prebuilt frontend image tag.
The frontend image is meant to be built in CI and pushed to GHCR, so VPS nodes only need to pull the image and start the containers (no local frontend build or dist mount required).
For staging deployments, set `NGINX_CONTAINER_NAME=nginx-staging` and `POCKETBASE_CONTAINER_NAME=pocketbase-staging` so only the staging containers carry the `-staging` suffix.

**GHCR tag versioning rules**

- `GHCR_TAG` **must stay in sync** with the version in `Cargo.toml` under `[package] version`.
- If you add a prefix (e.g. `v<version>` or any other prefix), keep the same underlying version from `Cargo.toml` and include the prefix in `GHCR_TAG`.
  - Example with `v` prefix: `GHCR_TAG=v{version}` (matches `Cargo.toml` version `{version}`).
- The deploy tool can generate tags from `GHCR_TAG_FORMAT` (e.g. `v{version}`) when `GHCR_TAG` is not set, using the build-time `CARGO_PKG_VERSION`.
- Staging and production use the same image tag; only container names are suffixed for staging.



### Standalone Remote Deploy Binary

For fastest startup and minimal dependency loading, use the dedicated binary:

```bash
cargo run --manifest-path tools/deploy-remote/Cargo.toml --release
```

It intentionally uses only Rust standard library (no clap/anyhow/dotenv/colored), and supports the same environment variables:
`DEPLOY_USER`, `DEPLOY_HOST`, `DEPLOY_PATH`, optional `DEPLOY_COMPOSE_FILE`, `SSH_KEY_FILE`, `SSH_KEY_PATH`, `SSH_KEY_NAME`, `GHCR_TAG`, `GHCR_TAG_FORMAT`, `DEPLOY_ENV`,
`NGINX_CONTAINER_NAME`, `POCKETBASE_CONTAINER_NAME`.

### Environment variables reference

Use this list to keep `.env` files and CI/CD configuration complete:

**Notes on frontend image variables**

- `GHCR_TAG` only controls the version tag portion of the frontend image in the compose template.
- `FRONTEND_IMAGE` overrides the entire image reference (registry/org/repo + tag), so it is useful when switching registries or repos.
- If you never need to override the image source and only want a single variable, you can remove `FRONTEND_IMAGE` from your compose file and rely solely on `GHCR_TAG` for versioning. Otherwise, keep both to preserve the override option.

| Variable | Used by | Purpose |
| --- | --- | --- |
| `DEPLOY_USER` | `deploy` binaries | SSH username for deployment. |
| `DEPLOY_HOST` | `deploy` binaries | SSH host/IP for deployment. |
| `DEPLOY_PATH` | `deploy` binaries | Remote deployment directory. |
| `DEPLOY_COMPOSE_FILE` | deploy (remote) | Docker Compose file name on the server (default: `docker-compose.deploy.yml`). |
| `SSH_KEY_FILE` | deploy | Full path to SSH private key (overrides `SSH_KEY_PATH` + `SSH_KEY_NAME`). |
| `SSH_KEY_PATH` | deploy | SSH key directory (default: `/root/.ssh`). |
| `SSH_KEY_NAME` | deploy | SSH key filename (default: `id_rsa`). |
| `SSH_KNOWN_HOSTS_FILE` | deploy-remote | Override `known_hosts` file path (default: `$HOME/.ssh/known_hosts` or `/root/.ssh/known_hosts`). |
| `DEPLOY_ENV` | deploy-remote | `staging`/`stage` appends `-staging` to container names. |
| `NGINX_CONTAINER_NAME` | deploy-remote | Override nginx container name (for staging/prod). |
| `POCKETBASE_CONTAINER_NAME` | deploy-remote | Override pocketbase container name (for staging/prod). |
| `GHCR_TAG` | deploy/tag tools | Explicit GHCR image tag (should match `Cargo.toml` version). |
| `GHCR_TAG_FORMAT` | deploy/tag tools | Tag template (supports `{version}` placeholder). |
| `GHCR_TAG_SUFFIX` | tag-production | Suffix to map staging vs production tags. |
| `GHCR_IMAGE` | tag-production | Full GHCR image name to promote (e.g., `ghcr.io/org/app`). |
| `PB_ENCRYPTION_KEY` | PocketBase | PocketBase encryption key in remote `.env`. |
| `NGINX_CONF_PATH` | nginx | Override nginx config path in remote compose file. |
| `FRONTEND_IMAGE` | nginx | Override frontend image in remote compose file. |
| `SKIP_TAILWIND` | build/test | Skip Tailwind build steps (set by test runner). |
| `DOCKER_CONTAINER` | test runner | Force container detection in `test-runner`. |
| `IFECARO_APP_VERSION` | build/tag tools | Override build-time version embedded in binaries. |

### Deployment Pipeline

The automated deployment process includes:

1. **Testing Phase**
   - ✅ Compilation checks
   - ✅ 97+ unit tests
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

The comprehensive test suite consists of **208+ tests total** covering everything from basic UI components to complex API integrations, dashboard functionality, reader mode auto-expansion, and performance optimizations:

```
src/
├── components/
│   ├── story_content_tests.rs              # Basic UI Tests (27 tests)
│   ├── story_content_advanced_tests.rs     # Advanced Feature Tests (28 tests)
│   └── story_content_api_integration_tests.rs # API Integration Tests (1 test)
├── pages/
│   ├── story_tests.rs                       # Story Logic Tests (16 tests)
│   └── dashboard_tests.rs                   # Dashboard Tests (31 tests)
├── services/
│   └── api_tests.rs                         # API Mock Tests (0 tests)

tests/
├── integration_tests.rs                    # Core Integration Tests (4 tests)
├── main_code_usage_example.rs             # Code Usage Examples (6 tests)
├── story_flow_tests.rs                    # Story Flow Tests (4 tests)
├── reader_mode_tests.rs                   # Reader Mode Unit Tests (10 tests) ✨ NEW
├── reader_mode_integration_tests.rs       # Reader Mode Integration Tests (6 tests) ✨ NEW
├── dashboard_tests.rs                      # Dashboard Unit Tests (31 tests)
├── dashboard_interaction_tests.rs          # Dashboard Interaction Tests (17 tests)
└── dashboard_benchmark_tests.rs           # Dashboard Performance Tests (11 tests)
```

### Test Coverage Summary

#### Functional Coverage
- ✅ **Basic UI Rendering**: Text display, choice lists, chapter titles (story_content_tests.rs)
- ✅ **Interactive Features**: Choice enable/disable, countdown timers, keyboard navigation (story_content_advanced_tests.rs)  
- ✅ **Responsive Design**: Multiple screen sizes, dark mode support (story_content_tests.rs)
- ✅ **Accessibility Features**: Semantic tags, focus management, WCAG compliance (story_content_tests.rs)
- ✅ **Data Processing**: JSON serialization, multilingual support (story_tests.rs + story_content_advanced_tests.rs)
- ✅ **API Integration**: Mock testing, error handling, data flow (story_content_api_integration_tests.rs + api_tests.rs)
- ✅ **Reader Mode Auto-Expansion**: Random choice selection, story path continuation, loop prevention (reader_mode_tests.rs + reader_mode_integration_tests.rs) ✨ **NEW**
- ✅ **Dashboard Management**: Content creation, editing, validation, multi-language support (dashboard_tests.rs)
- ✅ **Dashboard Interactions**: User workflows, form validation, state management, comprehensive language switching (dashboard_interaction_tests.rs)
- ✅ **Dashboard Performance**: Large dataset handling, concurrent operations, stress testing (dashboard_benchmark_tests.rs)
- ✅ **Advanced Form Features**: Real-time validation, dynamic button states, comprehensive language switching (dashboard_interaction_tests.rs)
- ✅ **Edge Cases**: Empty data, extremely long content, special characters (story_content_tests.rs + dashboard_tests.rs)
- ✅ **Performance Testing**: Large dataset rendering, memory optimization (story_content_tests.rs + dashboard_benchmark_tests.rs)
- ✅ **Regression Testing**: Protection against known issues (story_content_tests.rs + dashboard_tests.rs)
- ✅ **Core Business Logic**: Paragraph merging, countdown timers, reader mode (story_tests.rs)
- ✅ **End-to-End Flows**: Complete user journeys and integration scenarios (tests/ directory)

#### Complete Test Coverage Mapping

**1. Story Content UI Component Layer (56 tests)**
- **Visual Rendering Tests**: HTML structure, CSS classes, responsive design
- **User Interaction Tests**: Click events, keyboard navigation, state changes  
- **Accessibility Tests**: WCAG compliance, screen reader support, focus management
- **Performance Tests**: Large datasets, complex UI structures, rendering optimization
- **Edge Case Tests**: Unicode, emoji, special characters, extremely long content
- **Integration Tests**: Component props, event handling, state synchronization

**2. Dashboard Management Layer (59 tests)**
- **Unit Tests**: Data structures, language state, chapter state, paragraph state (31 tests)
- **Interaction Tests**: User workflows, form validation, content editing, comprehensive language switching (17 tests)
- **Performance Tests**: Large datasets, concurrent operations, stress testing (11 tests)
- **Component Tests**: Rendering, language switching, state management
- **Integration Tests**: API compatibility, data serialization, accessibility
- **Benchmark Tests**: Memory usage, rapid operations, massive datasets
- **Advanced Features**: Real-time form validation, dynamic button states, comprehensive UI/content language switching

**3. Story Business Logic Layer (16 tests)**
- **Core Algorithm Tests**: `merge_paragraphs_for_lang` function with all scenarios
- **Data Processing Tests**: Serialization, deserialization, validation  
- **Business Rule Tests**: Reader mode logic, chapter filtering, language processing
- **Integration Tests**: Complete data flow from raw data to processed output
- **Multilingual Tests**: Content processing for Chinese, English, Japanese

**4. API Service Layer (7 tests)**
- **CRUD Operations**: Success and failure scenarios for all API endpoints
- **Data Transformation**: API response processing and error handling
- **Mock Integration**: Realistic API simulation and edge case testing

**5. Integration & System Tests (20 tests)**
- **Context Integration**: Settings, story state, keyboard state management
- **Cross-Component Flow**: Complete user journey testing
- **Main Code Usage**: Direct testing of exported functions and components  
- **Story Flow Tests**: Reader mode vs normal mode, multi-chapter scenarios

**6. Reader Mode Expansion Tests (16 tests) ✨ NEW**
- **Unit Tests**: Paragraph merging logic, language filtering, random choice simulation (10 tests)
- **Integration Tests**: Story network expansion, multiple ending paths, loop prevention (6 tests)
- **Performance Tests**: Large story path handling, statistical distribution validation
- **Edge Case Tests**: Empty choice targets, single paragraphs, complex choice structures

#### Test Distribution Strategy

| Layer | Purpose | File Location | Test Count | Coverage Focus |
|-------|---------|---------------|------------|----------------|
| **UI Component** | Visual & Interactive | `story_content_*_tests.rs` | 56 | User interface, events, styling |
| **Dashboard Management** | Content Management | `dashboard_*_tests.rs` | 59 | Content creation, editing, validation, language switching |
| **Business Logic** | Core Algorithms | `story_tests.rs` | 16 | Data processing, business rules |
| **API Service** | Data Layer | `api_tests.rs` | 7 | External integrations, mocking |
| **System Integration** | End-to-End | `tests/` directory | 20 | Complete workflows, contexts |
| **Reader Mode** | Auto-Expansion | `reader_mode_*_tests.rs` | 16 | Story auto-expansion, random selection, loop prevention |
| **Total Coverage** | **Complete Application** | **All test files** | **208** | **100% functional coverage** |

### Dashboard Test Suite Details

The Dashboard component has its own comprehensive testing suite with 83 tests covering all aspects of content management:

#### Dashboard Unit Tests (31 tests)
- **Data Structure Tests**: Language state, chapter state, paragraph state
- **Component Tests**: Dashboard rendering, language switching
- **Integration Tests**: State management, localization, API compatibility
- **Form Validation Tests**: Content validation, error handling, real-time validation
- **Accessibility Tests**: Screen reader support, keyboard navigation
- **Performance Tests**: Large dataset handling, memory usage
- **Serialization Tests**: JSON processing, data integrity
- **UI State Tests**: Form state management, edit mode transitions
- **Button State Tests**: Dynamic submit button enable/disable logic

#### Dashboard Interaction Tests (17 tests)
- **User Workflow Tests**: Complete editing workflows, language switching
- **Form Validation Tests**: Real-time validation, error handling
- **Multi-language Tests**: Content switching, UI language independence
- **Edit Mode Tests**: Comprehensive language switching with content updates
- **Choice Management Tests**: Dynamic choice creation and validation
- **Edge Case Tests**: Error handling, invalid data, circular references

#### Dashboard UI Tests (24 tests) ✨ **New**
- **UI Rendering Tests**: Basic structure, form layout, responsive design
- **Language Tests**: Multi-language rendering, language switching
- **State Tests**: Edit mode layout, form areas, selector grids
- **Accessibility Tests**: Semantic structure, color contrast, responsive accessibility
- **Error State Tests**: Toast notifications, validation structure
- **Performance Tests**: Render performance, multiple language renders
- **Edge Case Tests**: Empty/invalid languages, special characters

#### Dashboard Benchmark Tests (11 tests)
- **Performance Benchmarks**: Large dataset processing, memory optimization
- **Stress Tests**: Concurrent operations, massive data handling
- **Optimization Tests**: Query performance, rendering efficiency

### Performance Improvements
- **Tailwind CSS Compilation**: Optimized to skip during tests
- **Build Cache Management**: Automatic cache clearing
- **Test Execution Speed**: 50-90% faster depending on test type
- **Dashboard Tests**: Optimized for large dataset handling

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
| **Reader Mode Unit** | 10 | `reader_mode_tests.rs` | Paragraph merging, language filtering, random choice simulation ✨ **NEW** |
| **Reader Mode Integration** | 6 | `reader_mode_integration_tests.rs` | Story network expansion, loop prevention, performance ✨ **NEW** |

### Reader Mode Test Suite Details ✨ **NEW**

The Reader Mode functionality has its own comprehensive testing suite with 16 tests covering all aspects of automatic story expansion:

#### Reader Mode Unit Tests (10 tests)
- **Paragraph Display Tests**: All expanded paragraphs shown in reader mode
- **Mode Comparison Tests**: Reader mode vs normal mode behavior differences  
- **Settings Chapter Tests**: Special handling for settings chapters
- **Language Filtering Tests**: Multi-language content processing
- **Random Selection Tests**: Statistical validation of choice distribution (1000 iterations)
- **Choice Structure Tests**: Complex choice validation and target handling
- **Edge Case Tests**: Empty paragraphs, single paragraphs, no valid choices

#### Reader Mode Integration Tests (6 tests)
- **Story Network Expansion**: Complete story path auto-expansion simulation
- **Multiple Ending Paths**: Testing different story completion scenarios
- **Loop Prevention Tests**: Visited tracking and circular reference prevention
- **Performance Tests**: Large story path handling (50+ paragraphs)
- **Language Consistency Tests**: Multi-language story expansion
- **Empty Choice Handling**: Graceful handling of paragraphs with no targets

#### Key Testing Achievements
- **Random Choice Validation**: 1000-iteration statistical testing ensuring proper distribution
- **Story Network Simulation**: Complete story graph traversal testing
- **Performance Benchmarking**: Linear story handling up to 50+ paragraphs
- **Edge Case Coverage**: Empty targets, loops, language mismatches, complex structures

### Test Execution Methods

```bash
# Using Rust test runner (recommended) - All options have Tailwind compilation optimization
cargo run --bin test-runner full      # Complete test suite (all 150 tests)
cargo run --bin test-runner quick     # Quick tests (compile + unit + integration)
cargo run --bin test-runner internal  # Container-optimized testing
cargo run --bin test-runner check     # Compile check only
cargo run --bin test-runner category <category>  # Specific test category
cargo run --bin test-runner bench     # Performance benchmark tests
cargo run --bin test-runner report    # Generate test report

# Available categories:
# compile, ui, advanced, mock-api, integration, unit, external, dashboard

# Dashboard-specific testing
cargo run --bin test-runner category dashboard  # All Dashboard tests (53 tests)

# Using deployment tool
cargo run --bin deploy test full      # Run complete test suite
cargo run --bin deploy dev            # Development mode (check + quick test)

# Direct cargo commands
cargo test --lib                      # Unit tests (136 tests)
cargo test --test integration_tests   # Core integration tests (4 tests)
cargo test --test main_code_usage_example  # Code usage examples (6 tests)
cargo test --test story_flow_tests    # Story flow tests (4 tests)
cargo test --test dashboard_tests     # Dashboard unit tests (28 tests)
cargo test --test dashboard_interaction_tests  # Dashboard interaction tests (14 tests)
cargo test --test dashboard_benchmark_tests    # Dashboard performance tests (11 tests)
```

### Code Quality Assurance

- ✅ **Compilation Check**: Zero warnings, zero errors
- ✅ **Type Safety**: Guaranteed by Rust type system  
- ✅ **Memory Safety**: No memory leak risks
- ✅ **Concurrency Safety**: Appropriate synchronization mechanisms
- ✅ **Performance Monitoring**: Test execution time tracking
- ✅ **Coverage Analysis**: Comprehensive functional coverage including Dashboard
- ✅ **Dashboard Testing**: Complete content management workflow validation

### Continuous Integration

- **Pre-commit**: Automatically execute complete test suite (150+ tests)
- **Pre-build**: Run all tests before production compilation
- **Docker Environment**: Ensures consistent test environment
- **Automated Deployment**: Tests must pass before deployment
- **Dashboard Validation**: Specialized Dashboard tests integrated into CI pipeline

### GitHub Actions CI Workflow

This repository includes a GitHub Actions workflow that validates compilation on every push and pull request by running `cargo check` and a release build with Tailwind compilation skipped in CI.

#### Usage Steps

1. **Enable GitHub Actions**  
   Make sure Actions are enabled in your GitHub repository settings.

2. **Push or open a pull request**  
   Any push to any branch, or a new pull request, will automatically trigger the workflow.

3. **Monitor CI results**  
   Go to the **Actions** tab in GitHub to see the job status and logs for `cargo check --all-targets` and `cargo build --release --all-targets`.

4. **Troubleshoot failures**  
   If the workflow fails, review the logs, fix issues locally, and push updates to rerun CI.

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
│   │   ├── story_content_advanced_tests.rs     # Advanced Feature Tests (28 tests)
│   │   └── story_content_api_integration_tests.rs # API Integration Tests (1 test)
│   ├── contexts/               # State management
│   ├── pages/                  # Page components
│   │   └── story_tests.rs      # Story Logic Tests (16 tests)
│   ├── services/               # API services
│   │   └── api_tests.rs        # API Mock Tests (0 tests)
│   └── main.rs                 # Application entry
├── tests/                      # External integration tests
│   ├── integration_tests.rs    # Core Integration Tests (4 tests)
│   ├── main_code_usage_example.rs # Code Usage Examples (6 tests)
│   └── story_flow_tests.rs     # Story Flow Tests (4 tests)
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

### Performance Optimizations

**Tailwind CSS Compilation Optimization**: All test commands now automatically skip Tailwind CSS compilation during tests, resulting in significant performance improvements:

- **Compile Check**: ~4.5s → ~1.2s (73% faster)
- **Unit Tests**: ~4s → ~2.9s (27% faster) 
- **Integration Tests**: ~5s → ~0.5s (90% faster)

**Build Cache Management**: Automatic build cache clearing ensures environment variables take impact properly.

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
   # Select option 5: ⚡ Development Mode (check + quick test)
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
   # Select option 3: 🏗 Build Project
   
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

- ✅ All tests must pass (83+ unit tests, 14+ integration tests)
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

// Page Logic - src/pages/story_tests.rs
#[cfg(test)]
mod page_logic_tests {
    use super::*;

    #[test]
    fn test_your_page_logic() {
        // Your page logic test
    }
}

// Integration Tests - tests/integration_tests.rs
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_your_integration() {
        // Your integration test
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

- **[Test Documentation](docs/Test-Documentation.md)**: Comprehensive test suite documentation with detailed descriptions of all 97+ tests
- **Test Categories**: Basic UI, Advanced Features, API Integration, Performance, Accessibility, Story Logic, Integration Tests
- **Test Architecture**: Multiple test files covering complete application functionality

---

**Built with ❤️ using Rust and Dioxus** 

### Total Test Distribution
- **Dashboard Tests**: 83 tests (45.6%) - Complete Dashboard functionality including UI rendering
- **Basic UI Tests**: 27 tests (14.8%) - Component rendering and interaction
- **Advanced Feature Tests**: 28 tests (15.4%) - Complex UI logic and performance
- **Story Logic Tests**: 16 tests (8.8%) - Core story processing with real data
- **Integration Tests**: 20 tests (11.0%) - End-to-End workflows
- **API Integration Tests**: 1 test (0.5%) - API data integration
- **Additional Library Tests**: 7 tests (3.8%) - Supporting functionality and utilities
- **Total**: **182+ tests** providing comprehensive coverage 

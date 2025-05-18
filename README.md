# Ifecaro Engine

Ifecaro is an interactive fiction (visual novel) website engine. It allows authors to create branching stories where readers can make choices that affect the narrative. The platform supports multi-path storytelling, player choices, internationalization (i18n), and a modern web interactive experience. Built with Rust and Dioxus, it delivers high performance and flexibility for both creators and readers.

**Live Demo:** [https://ifecaro.com/](https://ifecaro.com/)

Ifecaro is a web application engine built with Rust and Dioxus. This project uses Rust, WebAssembly, Tailwind CSS, and PocketBase as its main technology stack.

## Table of Contents

- [Requirements](#requirements)
- [Development Setup](#development-setup)
- [Local Development](#local-development)
- [Deployment Process](#deployment-process)
- [Database Setup](#database-setup)
- [Project Structure](#project-structure)
- [FAQ](#faq)

## Requirements

Before you start development, please make sure you have installed the following tools:

- [Rust](https://www.rust-lang.org/tools/install) (stable version)
- [Node.js](https://nodejs.org/) (for Tailwind CSS, or use Tailwind CSS standalone)
- [Dioxus CLI](https://dioxuslabs.com/docs/0.6/guide/en/getting_started/wasm.html)
- [Tailwind CSS](https://tailwindcss.com/docs/installation)
- [Docker](https://www.docker.com/get-started) (for PocketBase database)
- [Git](https://git-scm.com/downloads)

## Development Setup

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/Ifecaro-Engine.git
cd Ifecaro-Engine
```

### 2. Install Rust tools

Make sure you have the latest Rust toolchain:

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install cargo-watch
cargo install dioxus-cli
```

### 3. Install Node.js tools or Tailwind CSS Standalone

#### Option 1: Install via npm (requires Node.js)

```bash
npm install -g tailwindcss
```

#### Option 2: Download Tailwind CSS Standalone (no Node.js required)

You can download the standalone Tailwind CSS CLI from the [official release page](https://github.com/tailwindlabs/tailwindcss/releases/latest) and use it directly:

```bash
# Download the standalone binary (example for Linux x64)
curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
```

Then use `./tailwindcss` instead of `npx tailwindcss` in the following commands.

### 4. Setup PocketBase database

```bash
cd pocketbase
docker-compose up -d
```

The PocketBase admin UI will be available at `http://localhost:8090/_/`.

## Local Development

### Start the development server

In the project root directory, run:

```bash
# Start the development server with Dioxus CLI
dioxus serve

# Or use Cargo
cargo run
```

### Compile Tailwind CSS

```bash
# Watch and compile CSS in development mode
npx tailwindcss -i ./src/styles/input.css -o ./public/assets/tailwind.css --watch

# Production build
npx tailwindcss -i ./src/styles/input.css -o ./public/assets/tailwind.css --minify
```

### Code checking

```bash
cargo check
cargo clippy
```

## Deployment Process

### 1. Build WebAssembly

```bash
# Build with dioxus command
dioxus build --release

# Or use cargo
cargo build --release --target wasm32-unknown-unknown
```

### 2. Bundle CSS

```bash
npx tailwindcss -i ./src/styles/input.css -o ./dist/assets/tailwind.css --minify
```

### 3. Deploy PocketBase

To deploy PocketBase in production:

```bash
cd pocketbase
docker-compose -f docker-compose.production.yml up -d
```

### 4. Deploy frontend application

The compiled static files are in the `dist` directory and can be deployed to any static hosting service, such as Netlify, Vercel, or GitHub Pages.

## Database Setup

This project uses PocketBase as the backend database and authentication system. PocketBase is an open-source backend providing database, authentication, and file storage services.

### Initial Setup

1. Start PocketBase:
   ```bash
   cd pocketbase
   docker-compose up -d
   ```

2. Open your browser and visit `http://localhost:8090/_/`

3. Create an admin account

4. Import data schema (if available):
   In the PocketBase admin UI, go to Settings > Import Collections and upload the `pocketbase_schema.json` file from the project root.

## Project Structure

```
Ifecaro-Engine/
├── src/                     # Rust source code
├── public/                  # Static assets
├── pocketbase/              # PocketBase configuration
├── i18n/                    # i18n files
├── locales/                 # Localization resources
├── content/                 # Content files
├── dist/                    # Build output directory
├── Cargo.toml               # Rust project config
├── Cargo.lock               # Rust dependency lockfile
├── Dioxus.toml              # Dioxus config
├── index.html               # HTML template
├── tailwind.config.js       # Tailwind CSS config
└── README.md                # This file
```

## FAQ

### How to clean cache?

```bash
cargo clean
rm -rf dist
```

### How to update dependencies?

```bash
cargo update
```

### How to check for performance issues?

```bash
cargo build --release --target wasm32-unknown-unknown
dioxus build --release
```

Then use the browser's developer tools for performance profiling. 
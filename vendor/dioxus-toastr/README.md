# Dioxus Toastr

This crate provides a Dioxus-based toast notification system inspired by Toastr, enabling Rust/Dioxus apps to show typed, reactive notifications.

## Usage Example

```rust
use dioxus::prelude::*;
use dioxus_toastr::{ToastProvider, use_toast};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ToastProvider {
            Demo {}
        }
    }
}

#[component]
fn Demo() -> Element {
    let toast = use_toast();

    rsx! {
        button {
            onclick: move |_| {
                toast.success("Saved!", "Your changes are now live.");
            },
            "Show toast"
        }
    }
}
```

## Development Environment

### Requirements

- Rust stable toolchain (recommended via rustup)
- Cargo

### Setup

```bash
rustup update stable
```

Enter the crate directory:

```bash
cd dioxus_toastr
```

Run a compile check:

```bash
cargo check
```

## Development Workflow

1. **Edit modules**: Update the relevant files under `src/`.
2. **Compile check**: Run `cargo check` to validate types and syntax.
3. **Integration test**: Use `dioxus_toastr` in your Dioxus app to verify rendering and interactions.

Keep commits focused and scoped to a single feature or fix.

## Architecture

```
dioxus_toastr/
├── Cargo.toml
├── README.md
└── src/
    ├── handle.rs   # ToastHandle and use_toast API
    ├── lib.rs      # Public re-exports
    ├── provider.rs # ToastProvider and ToastItem components
    ├── store.rs    # ToastStore (signals + auto-dismiss)
    └── types.rs    # Toast/ToastRequest/ToastOptions/ToastKind
```

### Module Overview

- **types.rs**
  - Defines the toast types and data structures: `ToastKind`, `ToastOptions`, `ToastRequest`, and `Toast`.
- **store.rs**
  - Implements `ToastStore` using Dioxus `Signal`s to manage toasts and options, including auto-dismiss.
- **handle.rs**
  - Exposes `ToastHandle` and `use_toast()` with convenience helpers for `success/info/warning/error`.
- **provider.rs**
  - Renders the toast container via `ToastProvider` and handles user interactions in `ToastItem`.
- **lib.rs**
  - Re-exports the public API surface.

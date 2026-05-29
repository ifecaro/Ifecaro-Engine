use std::fs;
use std::path::Path;

fn read_repo_file(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", path.display());
    })
}

#[test]
fn dioxus_head_uses_root_relative_assets_for_deep_link_refreshes() {
    let config = read_repo_file("Dioxus.toml");

    for required_path in [
        "href=\"/assets/dioxus/Ifecaro-Engine.js\"",
        "href=\"/assets/dioxus/Ifecaro-Engine_bg.wasm\"",
        "href=\"/assets/fonts/NotoSansTC-Regular.woff2\"",
        "href=\"/manifest.json\"",
    ] {
        assert!(
            config.contains(required_path),
            "Dioxus.toml should include {required_path} so direct loads of nested routes fetch assets from the site root"
        );
    }

    for broken_relative_path in [
        "href=\"assets/dioxus/Ifecaro-Engine.js\"",
        "href=\"assets/dioxus/Ifecaro-Engine_bg.wasm\"",
        "href=\"assets/fonts/NotoSansTC-Regular.woff2\"",
        "href=\"manifest.json\"",
    ] {
        assert!(
            !config.contains(broken_relative_path),
            "Dioxus.toml should not contain {broken_relative_path}; relative assets resolve under /zh-TW/dashboard on hard refresh"
        );
    }
}

#[test]
fn checked_in_index_uses_root_relative_shell_assets() {
    let index = read_repo_file("index.html");

    assert!(
        index.contains("href=\"/manifest.json\""),
        "index.html should load the web manifest from the site root"
    );
    assert!(
        index.contains("href=\"/tailwind.css\""),
        "index.html should load the copied Tailwind stylesheet from the site root"
    );
    assert!(
        !index.contains("href=\"/assets/tailwind.css\""),
        "Dioxus copies public/tailwind.css to /tailwind.css, not /assets/tailwind.css"
    );
    assert!(
        !index.contains("href=\"manifest.json\""),
        "relative manifest links break on direct nested-route refreshes"
    );
    assert!(
        !index.contains("href=\"tailwind.css\""),
        "relative stylesheet links break on direct nested-route refreshes"
    );
}

#[test]
fn service_worker_precaches_copied_root_assets() {
    let service_worker = read_repo_file("public/sw.js");

    for required_path in ["'/tailwind.css'", "'/manifest.json'"] {
        assert!(
            service_worker.contains(required_path),
            "service worker should pre-cache {required_path} from the path emitted by Dioxus public asset copying"
        );
    }

    for broken_path in ["'/assets/tailwind.css'", "'/assets/manifest.json'"] {
        assert!(
            !service_worker.contains(broken_path),
            "service worker should not pre-cache missing root public files under /assets"
        );
    }
}

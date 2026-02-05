use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=../../Cargo.toml");

    let Ok(manifest) = fs::read_to_string("../../Cargo.toml") else {
        return;
    };

    if let Some(version) = parse_package_version(&manifest) {
        println!("cargo:rustc-env=IFECARO_APP_VERSION={version}");
    }
}

fn parse_package_version(manifest: &str) -> Option<String> {
    let mut in_package = false;

    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            in_package = line == "[package]";
            continue;
        }

        if !in_package {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        if key.trim() != "version" {
            continue;
        }

        let version = value.trim();
        if version.len() >= 2 && version.starts_with('"') && version.ends_with('"') {
            return Some(version[1..version.len() - 1].to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::parse_package_version;

    #[test]
    fn parses_package_version() {
        let manifest = r#"
            [package]
            name = "ifecaro"
            version = "1.2.3"

            [dependencies]
            anyhow = "1"
        "#;

        assert_eq!(parse_package_version(manifest), Some("1.2.3".to_string()));
    }

    #[test]
    fn skips_workspace_version() {
        let manifest = r#"
            [workspace.package]
            version = "9.9.9"

            [package]
            name = "ifecaro"
        "#;

        assert_eq!(parse_package_version(manifest), None);
    }
}

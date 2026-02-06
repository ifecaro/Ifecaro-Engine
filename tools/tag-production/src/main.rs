use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<(), String> {
    load_env_file();

    let app_version = resolve_app_version();
    let base_tag = resolve_base_ghcr_tag(app_version);
    let suffix = resolve_ghcr_suffix();
    let image = required_env("GHCR_IMAGE")?;

    let (production_tag, staging_tag) = resolve_tags(&base_tag, &suffix);
    let staging_ref = format!("{}:{}", image, staging_tag);
    let production_ref = format!("{}:{}", image, production_tag);

    run_docker(["pull", &staging_ref])?;
    run_docker(["tag", &staging_ref, &production_ref])?;
    run_docker(["push", &production_ref])?;

    println!(
        "✅ Promoted {} -> {}",
        staging_ref,
        production_ref
    );

    Ok(())
}

fn resolve_tags(base_tag: &str, suffix: &str) -> (String, String) {
    if suffix.is_empty() {
        return (base_tag.to_string(), base_tag.to_string());
    }

    if let Some(stripped) = base_tag.strip_suffix(suffix) {
        let production_tag = stripped.to_string();
        let staging_tag = format!("{}{}", production_tag, suffix);
        return (production_tag, staging_tag);
    }

    (base_tag.to_string(), format!("{}{}", base_tag, suffix))
}

fn resolve_base_ghcr_tag(cargo_version: &str) -> String {
    if let Ok(existing_tag) = env::var("GHCR_TAG") {
        return existing_tag;
    }

    if let Ok(format) = env::var("GHCR_TAG_FORMAT") {
        if format.contains("{version}") {
            return format.replace("{version}", cargo_version);
        }
        return format + cargo_version;
    }

    cargo_version.to_string()
}

fn resolve_ghcr_suffix() -> String {
    if let Ok(suffix) = env::var("GHCR_TAG_SUFFIX") {
        if !suffix.trim().is_empty() {
            return suffix;
        }
    }

    "-staging".to_string()
}

fn resolve_app_version() -> &'static str {
    option_env!("IFECARO_APP_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("❌ Missing {} environment variable", name))
}

fn run_docker<I>(args: I) -> Result<(), String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let status = Command::new("docker")
        .args(args.into_iter().map(|item| item.as_ref().to_string()))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("failed to run docker: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("❌ Docker command failed".to_string())
    }
}

fn load_env_file() {
    let path = Path::new(".env");
    if !path.exists() {
        return;
    }

    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let key = key.trim();
        if key.is_empty() || env::var_os(key).is_some() {
            continue;
        }

        let parsed = parse_env_value(value.trim());
        // SAFETY: This binary is single-threaded when loading .env and does not
        // concurrently access environment variables from other threads.
        unsafe { env::set_var(key, parsed) };
    }
}

fn parse_env_value(raw: &str) -> String {
    let value = raw.trim();
    if value.len() >= 2 {
        if (value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\''))
        {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::resolve_tags;

    #[test]
    fn resolves_tags_with_suffix() {
        let (production, staging) = resolve_tags("0.15.1", "-staging");
        assert_eq!(production, "0.15.1");
        assert_eq!(staging, "0.15.1-staging");
    }

    #[test]
    fn resolves_tags_when_base_includes_suffix() {
        let (production, staging) = resolve_tags("0.15.1-staging", "-staging");
        assert_eq!(production, "0.15.1");
        assert_eq!(staging, "0.15.1-staging");
    }
}

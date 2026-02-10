use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<(), String> {
    load_env_file();

    let app_version = resolve_app_version();
    let ghcr_tag = resolve_base_ghcr_tag(app_version);
    let container_suffix = resolve_container_suffix();
    let compose_project_name = resolve_compose_project_name(&container_suffix);
    let frontend_container_name = resolve_frontend_container_name(&container_suffix);
    let nginx_container_name = resolve_nginx_container_name(&container_suffix);
    let pocketbase_container_name = resolve_pocketbase_container_name(&container_suffix);

    let deploy_user = required_env("DEPLOY_USER")?;
    let deploy_host = required_env("DEPLOY_HOST")?;
    let deploy_path = required_env("DEPLOY_PATH")?;
    let deploy_compose_file =
        env::var("DEPLOY_COMPOSE_FILE").unwrap_or_else(|_| "docker-compose.deploy.yml".to_string());
    let ssh_key_file = resolve_ssh_key_file();
    let known_hosts_file = resolve_known_hosts_file();

    let remote_command = format!(
        "cd {} && GHCR_TAG={} FRONTEND_CONTAINER_NAME={} NGINX_CONTAINER_NAME={} POCKETBASE_CONTAINER_NAME={} docker compose -p {} -f {} pull && GHCR_TAG={} FRONTEND_CONTAINER_NAME={} NGINX_CONTAINER_NAME={} POCKETBASE_CONTAINER_NAME={} docker compose -p {} -f {} up -d",
        deploy_path,
        shell_escape(&ghcr_tag),
        shell_escape(&frontend_container_name),
        shell_escape(&nginx_container_name),
        shell_escape(&pocketbase_container_name),
        shell_escape(&compose_project_name),
        deploy_compose_file,
        shell_escape(&ghcr_tag),
        shell_escape(&frontend_container_name),
        shell_escape(&nginx_container_name),
        shell_escape(&pocketbase_container_name),
        shell_escape(&compose_project_name),
        deploy_compose_file
    );

    let status = Command::new("ssh")
        .args([
            "-i",
            &ssh_key_file,
            "-o",
            &format!("UserKnownHostsFile={}", known_hosts_file),
            "-o",
            "StrictHostKeyChecking=yes",
            "-o",
            "PasswordAuthentication=no",
            "-o",
            "PubkeyAuthentication=yes",
            "-o",
            "ConnectTimeout=30",
            &format!("{}@{}", deploy_user, deploy_host),
            &remote_command,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("failed to run ssh: {}", e))?;

    if status.success() {
        println!("✅ Remote VPS deployment completed (GHCR pull + docker compose up)");
        Ok(())
    } else {
        Err("❌ Remote VPS deployment failed".to_string())
    }
}

fn resolve_known_hosts_file() -> String {
    if let Ok(known_hosts_file) = env::var("SSH_KNOWN_HOSTS_FILE") {
        if !known_hosts_file.trim().is_empty() {
            return known_hosts_file;
        }
    }

    if let Ok(home) = env::var("HOME") {
        if !home.trim().is_empty() {
            return format!("{}/.ssh/known_hosts", home);
        }
    }

    "/root/.ssh/known_hosts".to_string()
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("❌ Missing {} environment variable", name))
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

fn resolve_container_suffix() -> String {
    if is_production_enabled() {
        String::new()
    } else {
        "-staging".to_string()
    }
}

fn resolve_frontend_container_name(container_suffix: &str) -> String {
    format!("frontend{}", container_suffix)
}

fn resolve_nginx_container_name(container_suffix: &str) -> String {
    format!("nginx{}", container_suffix)
}

fn resolve_pocketbase_container_name(container_suffix: &str) -> String {
    format!("pocketbase{}", container_suffix)
}

fn resolve_compose_project_name(container_suffix: &str) -> String {
    if let Ok(compose_project_name) = env::var("DEPLOY_COMPOSE_PROJECT_NAME") {
        if !compose_project_name.trim().is_empty() {
            return compose_project_name;
        }
    }

    if container_suffix.is_empty() {
        "ifecaro-production".to_string()
    } else {
        format!("ifecaro{}", container_suffix)
    }
}

fn is_production_enabled() -> bool {
    let Ok(value) = env::var("PRODUCTION") else {
        return false;
    };

    is_truthy_production_value(&value)
}

fn is_truthy_production_value(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "true" | "1" | "yes" | "on"
    )
}

fn resolve_app_version() -> &'static str {
    option_env!("IFECARO_APP_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
}

fn resolve_ssh_key_file() -> String {
    if let Ok(ssh_key_file) = env::var("SSH_KEY_FILE") {
        if !ssh_key_file.trim().is_empty() {
            return ssh_key_file;
        }
    }

    let ssh_key_path = env::var("SSH_KEY_PATH").unwrap_or_else(|_| "/root/.ssh".to_string());
    let ssh_key_name = env::var("SSH_KEY_NAME").unwrap_or_else(|_| "id_rsa".to_string());
    format!("{}/{}", ssh_key_path, ssh_key_name)
}

fn shell_escape(value: &str) -> String {
    let escaped = value.replace('\'', "'\"'\"'");
    format!("'{}'", escaped)
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
    use super::*;

    #[test]
    fn truthy_production_values_enable_production() {
        for truthy in ["true", "TRUE", "1", "yes", "on", " On "] {
            assert!(
                is_truthy_production_value(truthy),
                "expected truthy value: {}",
                truthy
            );
        }
    }

    #[test]
    fn non_truthy_production_values_use_staging() {
        for non_truthy in ["false", "0", "staging", "", "prod", "enabled"] {
            assert!(
                !is_truthy_production_value(non_truthy),
                "expected non-truthy value: {}",
                non_truthy
            );
        }
    }

    #[test]
    fn compose_project_name_defaults_to_staging_project() {
        assert_eq!(
            resolve_compose_project_name("-staging"),
            "ifecaro-staging".to_string()
        );
    }

    #[test]
    fn compose_project_name_defaults_to_production_project() {
        assert_eq!(
            resolve_compose_project_name(""),
            "ifecaro-production".to_string()
        );
    }

    #[test]
    fn compose_project_name_supports_override() {
        // SAFETY: test-only scoped environment mutation.
        unsafe { env::set_var("DEPLOY_COMPOSE_PROJECT_NAME", "custom-ifecaro") };

        assert_eq!(
            resolve_compose_project_name("-staging"),
            "custom-ifecaro".to_string()
        );

        // SAFETY: test-only cleanup of environment variable.
        unsafe { env::remove_var("DEPLOY_COMPOSE_PROJECT_NAME") };
    }

    #[test]
    fn container_names_include_staging_suffix() {
        assert_eq!(
            resolve_frontend_container_name("-staging"),
            "frontend-staging"
        );
        assert_eq!(resolve_nginx_container_name("-staging"), "nginx-staging");
        assert_eq!(
            resolve_pocketbase_container_name("-staging"),
            "pocketbase-staging"
        );
    }
}

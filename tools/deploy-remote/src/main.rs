use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<(), String> {
    load_env_file();

    let app_version = resolve_app_version();
    let ghcr_tag = resolve_ghcr_tag(app_version);

    let deploy_user = required_env("DEPLOY_USER")?;
    let deploy_host = required_env("DEPLOY_HOST")?;
    let deploy_path = required_env("DEPLOY_PATH")?;
    let deploy_compose_file =
        env::var("DEPLOY_COMPOSE_FILE").unwrap_or_else(|_| "docker-compose.deploy.yml".to_string());
    let ssh_key_file = resolve_ssh_key_file();

    let remote_command = format!(
        "cd {} && GHCR_TAG={} docker compose -f {} pull && GHCR_TAG={} docker compose -f {} up -d",
        deploy_path,
        shell_escape(&ghcr_tag),
        deploy_compose_file,
        shell_escape(&ghcr_tag),
        deploy_compose_file
    );

    let status = Command::new("ssh")
        .args([
            "-i",
            &ssh_key_file,
            "-o",
            "UserKnownHostsFile=/root/.ssh/known_hosts",
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

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("❌ Missing {} environment variable", name))
}

fn resolve_ghcr_tag(cargo_version: &str) -> String {
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

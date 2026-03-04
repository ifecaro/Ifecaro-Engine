use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::Deserialize;

fn main() -> Result<(), String> {
    load_env_file();

    let app_version = resolve_app_version();
    let ghcr_tag = resolve_base_ghcr_tag(app_version);
    let container_suffix = resolve_container_suffix();
    let compose_project_name = resolve_compose_project_name(&container_suffix);
    let frontend_container_name = resolve_frontend_container_name(&container_suffix);
    let nginx_container_name = resolve_nginx_container_name(&container_suffix);
    let pocketbase_container_name = resolve_pocketbase_container_name(&container_suffix);
    let deploy_environment = resolve_deploy_environment(&container_suffix);
    let api_url = resolve_api_url(&deploy_environment);
    let nginx_conf_path = resolve_nginx_conf_path(&deploy_environment);
    let expected_git_sha = required_expected_git_sha()?;

    let deploy_user = required_env("DEPLOY_USER")?;
    let deploy_host = required_env("DEPLOY_HOST")?;
    let deploy_path = required_env("DEPLOY_PATH")?;
    let deploy_compose_file = resolve_deploy_compose_file(&deploy_environment);
    let ssh_key_file = resolve_ssh_key_file();
    let known_hosts_file = resolve_known_hosts_file();

    let frontend_image = resolve_frontend_image(&deploy_environment);

    let remote_pull_command = format!(
        "cd {} && GHCR_TAG={} FRONTEND_IMAGE={} VITE_APP_ENV={} VITE_BASE_API_URL={} NGINX_CONF_PATH={} FRONTEND_CONTAINER_NAME={} NGINX_CONTAINER_NAME={} POCKETBASE_CONTAINER_NAME={} docker compose -p {} -f {} pull",
        deploy_path,
        shell_escape(&ghcr_tag),
        shell_escape(&frontend_image),
        shell_escape(&deploy_environment),
        shell_escape(&api_url),
        shell_escape(&nginx_conf_path),
        shell_escape(&frontend_container_name),
        shell_escape(&nginx_container_name),
        shell_escape(&pocketbase_container_name),
        shell_escape(&compose_project_name),
        deploy_compose_file
    );

    let pull_status = run_ssh_command(
        &ssh_key_file,
        &known_hosts_file,
        &deploy_user,
        &deploy_host,
        &remote_pull_command,
    )?;

    if !pull_status.success() {
        return Err("❌ Remote VPS image pull failed".to_string());
    }

    let remote_frontend_pull_command = format!(
        "docker pull {}",
        shell_escape(&frontend_image)
    );

    let frontend_pull_output = run_ssh_command_with_output(
        &ssh_key_file,
        &known_hosts_file,
        &deploy_user,
        &deploy_host,
        &remote_frontend_pull_command,
    )?;

    if !frontend_pull_output.status.success() {
        let stderr_preview = safe_response_preview(&String::from_utf8_lossy(&frontend_pull_output.stderr));
        return Err(format!(
            "❌ Remote frontend image pull failed for {} (status: {}). stderr: {}",
            frontend_image, frontend_pull_output.status, stderr_preview
        ));
    }

    verify_remote_image_git_sha(
        &ssh_key_file,
        &known_hosts_file,
        &deploy_user,
        &deploy_host,
        &frontend_image,
        &expected_git_sha,
    )?;

    let remote_up_command = format!(
        "cd {} && GHCR_TAG={} FRONTEND_IMAGE={} VITE_APP_ENV={} VITE_BASE_API_URL={} NGINX_CONF_PATH={} FRONTEND_CONTAINER_NAME={} NGINX_CONTAINER_NAME={} POCKETBASE_CONTAINER_NAME={} docker compose -p {} -f {} up -d",
        deploy_path,
        shell_escape(&ghcr_tag),
        shell_escape(&frontend_image),
        shell_escape(&deploy_environment),
        shell_escape(&api_url),
        shell_escape(&nginx_conf_path),
        shell_escape(&frontend_container_name),
        shell_escape(&nginx_container_name),
        shell_escape(&pocketbase_container_name),
        shell_escape(&compose_project_name),
        deploy_compose_file
    );

    let up_status = run_ssh_command(
        &ssh_key_file,
        &known_hosts_file,
        &deploy_user,
        &deploy_host,
        &remote_up_command,
    )?;

    if !up_status.success() {
        return Err("❌ Remote VPS deployment failed".to_string());
    }

    if deploy_environment == "staging" {
        rewrite_staging_base_url_on_remote(
            &ssh_key_file,
            &known_hosts_file,
            &deploy_user,
            &deploy_host,
            &frontend_container_name,
        )?;
    }

    println!(
        "✅ Remote VPS deployment completed (GHCR pull + image SHA verify + docker compose up)"
    );
    Ok(())
}

fn run_ssh_command_with_output(
    ssh_key_file: &str,
    known_hosts_file: &str,
    deploy_user: &str,
    deploy_host: &str,
    remote_command: &str,
) -> Result<std::process::Output, String> {
    Command::new("ssh")
        .args([
            "-i",
            ssh_key_file,
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
            remote_command,
        ])
        .output()
        .map_err(|e| format!("failed to run ssh: {}", e))
}

fn rewrite_staging_base_url_on_remote(
    ssh_key_file: &str,
    known_hosts_file: &str,
    deploy_user: &str,
    deploy_host: &str,
    frontend_container_name: &str,
) -> Result<(), String> {
    let remote_command = build_staging_base_url_rewrite_command(frontend_container_name);
    let status = run_ssh_command(
        ssh_key_file,
        known_hosts_file,
        deploy_user,
        deploy_host,
        &remote_command,
    )?;

    if !status.success() {
        return Err("❌ Failed to rewrite staging API base URL in remote frontend container".to_string());
    }

    println!(
        "✅ Rewrote staging API base URL in remote frontend container ({})",
        frontend_container_name
    );
    Ok(())
}

fn build_staging_base_url_rewrite_command(frontend_container_name: &str) -> String {
    format!(
        r#"docker exec {} sh -lc 'index=/dist/index.html && [ -f "$index" ] && sed -i -e "s|https://ifecaro.com/db/api|https://ifecaro.com/staging/db/api|g" -e "s|\"/db/api\"|\"/staging/db/api\"|g" -e "s|\'/db/api\'|\'/staging/db/api\'|g" -e "s|\"/assets/|\"/staging/assets/|g" -e "s|\'/assets/|\'/staging/assets/|g" -e "s|=/assets/|=/staging/assets/|g" "$index"'"#,
        shell_escape(frontend_container_name)
    )
}
fn run_ssh_command(
    ssh_key_file: &str,
    known_hosts_file: &str,
    deploy_user: &str,
    deploy_host: &str,
    remote_command: &str,
) -> Result<std::process::ExitStatus, String> {
    Command::new("ssh")
        .args([
            "-i",
            ssh_key_file,
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
            remote_command,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("failed to run ssh: {}", e))
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

fn required_expected_git_sha() -> Result<String, String> {
    if let Ok(value) = env::var("DEPLOY_EXPECTED_GIT_SHA") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    if let Ok(value) = env::var("GITHUB_SHA") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    Err(
        "❌ Missing DEPLOY_EXPECTED_GIT_SHA (or GITHUB_SHA) for deployed version verification"
            .to_string(),
    )
}

fn verify_remote_image_git_sha(
    ssh_key_file: &str,
    known_hosts_file: &str,
    deploy_user: &str,
    deploy_host: &str,
    frontend_image: &str,
    expected_git_sha: &str,
) -> Result<(), String> {
    let remote_command = format!(
        "docker run --rm --entrypoint cat {} /dist/version.json",
        shell_escape(frontend_image)
    );

    let output = Command::new("ssh")
        .args([
            "-i",
            ssh_key_file,
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
        .output()
        .map_err(|e| format!("❌ Failed to inspect remote frontend image: {}", e))?;

    if !output.status.success() {
        let stderr_preview = safe_response_preview(&String::from_utf8_lossy(&output.stderr));
        return Err(format!(
            "❌ Failed to inspect remote frontend image {} (status: {}). stderr: {}",
            frontend_image, output.status, stderr_preview
        ));
    }

    let payload = String::from_utf8(output.stdout).map_err(|e| {
        format!(
            "❌ Failed to parse remote image version payload as UTF-8 from {}: {}",
            frontend_image, e
        )
    })?;

    let deployed_sha = parse_git_sha_from_version_payload(&payload, frontend_image)?;
    if deployed_sha != expected_git_sha {
        return Err(format!(
            "❌ Pulled image SHA mismatch: expected {}, got {} from image {}",
            expected_git_sha, deployed_sha, frontend_image
        ));
    }

    println!(
        "✅ Verified pulled image git SHA ({}) from {}",
        deployed_sha, frontend_image
    );
    Ok(())
}

#[derive(Deserialize)]
struct VersionPayload {
    git_sha: Option<String>,
    #[allow(dead_code)]
    build_time: Option<String>,
    #[allow(dead_code)]
    app_version: Option<String>,
}

fn parse_git_sha_from_version_payload(
    body: &str,
    version_endpoint_url: &str,
) -> Result<String, String> {
    let response_preview = safe_response_preview(body);
    let payload = serde_json::from_str::<VersionPayload>(body).map_err(|e| {
        format!(
            "❌ Failed to parse version payload from {}: {}. Response preview: {}",
            version_endpoint_url, e, response_preview
        )
    })?;

    let git_sha = payload.git_sha.unwrap_or_default();
    if git_sha.trim().is_empty() {
        return Err(format!(
            "❌ Failed to parse version payload from {}: missing or empty git_sha. Response preview: {}",
            version_endpoint_url,
            response_preview
        ));
    }

    Ok(git_sha)
}

fn safe_response_preview(response_body: &str) -> String {
    let normalized = response_body
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let max_len = 200;
    let preview: String = normalized.chars().take(max_len).collect();

    if normalized.chars().count() > max_len {
        format!("{}…", preview)
    } else {
        preview
    }
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


fn resolve_deploy_compose_file(deploy_environment: &str) -> String {
    if let Ok(value) = env::var("DEPLOY_COMPOSE_FILE") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    if deploy_environment == "production" {
        "docker-compose.production.yml".to_string()
    } else {
        "docker-compose.staging.yml".to_string()
    }
}

fn resolve_deploy_environment(container_suffix: &str) -> String {
    if container_suffix.is_empty() {
        "production".to_string()
    } else {
        "staging".to_string()
    }
}

fn resolve_frontend_image(deploy_environment: &str) -> String {
    if let Ok(frontend_image) = env::var("FRONTEND_IMAGE") {
        if !frontend_image.trim().is_empty() {
            return frontend_image;
        }
    }

    let default_tag = if deploy_environment == "production" {
        "latest"
    } else {
        "latest-staging"
    };

    format!("ghcr.io/muchobien/ifecaro-engine:{}", default_tag)
}

fn resolve_api_url(deploy_environment: &str) -> String {
    let env_key = if deploy_environment == "production" {
        "PRODUCTION_API_URL"
    } else {
        "STAGING_API_URL"
    };

    if let Ok(api_url) = env::var(env_key) {
        if !api_url.trim().is_empty() {
            return api_url;
        }
    }

    if deploy_environment == "production" {
        "https://ifecaro.com/db/api".to_string()
    } else {
        "https://ifecaro.com/staging/db/api".to_string()
    }
}

fn resolve_nginx_conf_path(deploy_environment: &str) -> String {
    if let Ok(nginx_conf_path) = env::var("NGINX_CONF_PATH") {
        if !nginx_conf_path.trim().is_empty() {
            return nginx_conf_path;
        }
    }

    if deploy_environment == "production" {
        "./nginx/conf.d".to_string()
    } else {
        "./nginx/conf.d/staging".to_string()
    }
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
    let Ok(value) = env::var("DEPLOY_TO_PRODUCTION") else {
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
    use std::env;

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
    fn resolve_deploy_environment_matches_suffix() {
        assert_eq!(resolve_deploy_environment(""), "production".to_string());
        assert_eq!(
            resolve_deploy_environment("-staging"),
            "staging".to_string()
        );
    }

    #[test]
    fn production_toggle_defaults_to_staging() {
        // SAFETY: test-only cleanup to ensure deterministic default behavior.
        unsafe { env::remove_var("DEPLOY_TO_PRODUCTION") };
        assert_eq!(resolve_container_suffix(), "-staging");
    }

    #[test]
    fn production_toggle_uses_production_when_enabled() {
        // SAFETY: test-only scoped environment mutation.
        unsafe { env::set_var("DEPLOY_TO_PRODUCTION", "true") };
        assert_eq!(resolve_container_suffix(), "");

        // SAFETY: test-only cleanup of environment variable.
        unsafe { env::remove_var("DEPLOY_TO_PRODUCTION") };
    }

    #[test]
    fn resolve_api_url_defaults_by_environment() {
        // SAFETY: tests are single-threaded in this binary and this mutation is scoped to test process.
        unsafe {
            env::remove_var("STAGING_API_URL");
            env::remove_var("PRODUCTION_API_URL");
        }

        assert_eq!(
            resolve_api_url("staging"),
            "https://ifecaro.com/staging/db/api".to_string()
        );
        assert_eq!(
            resolve_api_url("production"),
            "https://ifecaro.com/db/api".to_string()
        );
    }

    #[test]
    fn resolve_nginx_conf_path_defaults_by_environment() {
        // SAFETY: tests are single-threaded in this binary and this mutation is scoped to test process.
        unsafe {
            env::remove_var("NGINX_CONF_PATH");
        }

        assert_eq!(
            resolve_nginx_conf_path("staging"),
            "./nginx/conf.d/staging".to_string()
        );
        assert_eq!(
            resolve_nginx_conf_path("production"),
            "./nginx/conf.d".to_string()
        );
    }

    #[test]
    fn compose_project_name_defaults_to_staging_project() {
        // SAFETY: test-only cleanup to ensure deterministic default behavior.
        unsafe { env::remove_var("DEPLOY_COMPOSE_PROJECT_NAME") };

        assert_eq!(
            resolve_compose_project_name("-staging"),
            "ifecaro-staging".to_string()
        );
    }

    #[test]
    fn compose_project_name_defaults_to_production_project() {
        // SAFETY: test-only cleanup to ensure deterministic default behavior.
        unsafe { env::remove_var("DEPLOY_COMPOSE_PROJECT_NAME") };

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

    #[test]
    fn parse_git_sha_from_version_payload_succeeds_with_normal_json() {
        let json = r#"{
  "git_sha": "abc123",
  "build_time": "2026-01-01T00:00:00Z"
}"#;
        assert_eq!(
            parse_git_sha_from_version_payload(json, "https://ifecaro.com/version.json"),
            Ok("abc123".to_string())
        );
    }

    #[test]
    fn parse_git_sha_from_version_payload_fails_when_git_sha_missing() {
        let json = r#"{ "build_time": "2026-01-01T00:00:00Z" }"#;
        let result = parse_git_sha_from_version_payload(json, "https://ifecaro.com/version.json");

        let err = result.expect_err("expected missing git_sha to fail");
        assert!(err.contains("https://ifecaro.com/version.json"));
        assert!(err.contains("missing or empty git_sha"));
        assert!(err.contains("Response preview"));
    }

    #[test]
    fn staging_base_url_rewrite_command_targets_dist_index_and_container() {
        let command = build_staging_base_url_rewrite_command("frontend-staging");

        assert!(command.contains("docker exec 'frontend-staging' sh -lc '"));
        assert!(command.contains("index=/dist/index.html"));
        assert!(command.contains("\"$index\""));
        assert!(command.contains("https://ifecaro.com/staging/db/api"));
        assert!(command.contains("/staging/db/api"));
        assert!(command.contains("/staging/assets/"));
        assert!(!command.contains("\\\n"));
    }

    #[test]
    fn parse_git_sha_from_version_payload_fails_for_non_json_response() {
        let html = "\n\n<html>\n<body>   502 bad gateway   </body>\n</html>\n";
        let result = parse_git_sha_from_version_payload(html, "https://ifecaro.com/version.json");

        let err = result.expect_err("expected non-json response to fail");
        assert!(err.contains("https://ifecaro.com/version.json"));
        assert!(err.contains("Response preview: <html> <body> 502 bad gateway </body> </html>"));
    }

    #[test]
    fn resolve_deploy_compose_file_defaults_by_environment() {
        unsafe {
            env::remove_var("DEPLOY_COMPOSE_FILE");
        }

        assert_eq!(
            resolve_deploy_compose_file("production"),
            "docker-compose.production.yml"
        );
        assert_eq!(
            resolve_deploy_compose_file("staging"),
            "docker-compose.staging.yml"
        );
    }

}

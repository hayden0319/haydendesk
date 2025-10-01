/// API Client with Automatic Server Failover
/// Replaces api_client.rs with multi-server support

use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use crate::api_server_config::{get_api_server, mark_server_failed, mark_server_success};

// Global HTTP client with connection pooling
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(std::time::Duration::from_secs(90))
        .connect_timeout(std::time::Duration::from_secs(5))
        .tcp_keepalive(std::time::Duration::from_secs(60))
        .build()
        .expect("Failed to create HTTP client")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySettingsResponse {
    pub status: String,
    pub can_modify_settings: bool,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
struct VerifySettingsRequest {
    device_id: String,
    username: String,
    password: String,
}

/// Verify user credentials for modifying settings
/// Automatically tries fallback servers if primary fails
pub async fn verify_settings_password(
    username: &str,
    password: &str,
) -> Result<VerifySettingsResponse, String> {
    let device_id = hbb_common::config::Config::get_id();

    let request = VerifySettingsRequest {
        device_id,
        username: username.to_string(),
        password: password.to_string(),
    };

    // Try with automatic failover
    retry_with_failover(|server_url| async move {
        let url = format!("{}/api/verify-settings", server_url);

        let response = HTTP_CLIENT
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            let result: VerifySettingsResponse = response
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;

            if result.can_modify_settings {
                Ok(result)
            } else {
                Err("Account does not have permission to modify settings".to_string())
            }
        } else {
            let error: serde_json::Value = response
                .json()
                .await
                .unwrap_or_else(|_| serde_json::json!({"message": "Unknown error"}));

            Err(error["message"].as_str().unwrap_or("Authentication failed").to_string())
        }
    })
    .await
}

#[derive(Debug, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
    device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub status: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub can_modify_settings: Option<bool>,
}

/// Login and get JWT token
/// Automatically tries fallback servers if primary fails
pub async fn login(username: &str, password: &str) -> Result<LoginResponse, String> {
    let device_id = hbb_common::config::Config::get_id();

    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
        device_id,
    };

    // Try with automatic failover
    retry_with_failover(|server_url| async move {
        let url = format!("{}/api/login", server_url);

        let response = HTTP_CLIENT
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    })
    .await
}

/// Retry logic with automatic server failover
/// Tries all available servers until one succeeds
async fn retry_with_failover<F, Fut, T>(mut operation: F) -> Result<T, String>
where
    F: FnMut(String) -> Fut,
    Fut: std::future::Future<Output = Result<T, String>>,
{
    const MAX_RETRIES: u32 = 3;
    let mut last_error = String::new();

    for attempt in 1..=MAX_RETRIES {
        // Get best available server
        let server_url = match get_api_server().await {
            Ok(url) => url,
            Err(e) => {
                log::error!("Failed to get API server: {}", e);
                return Err(e);
            }
        };

        log::debug!("Attempt {}/{} using server: {}", attempt, MAX_RETRIES, server_url);

        // Try the operation
        match operation(server_url.clone()).await {
            Ok(result) => {
                // Success! Mark server as healthy
                mark_server_success(&server_url).await;
                return Ok(result);
            }
            Err(e) => {
                log::warn!(
                    "Request failed on {} (attempt {}): {}",
                    server_url,
                    attempt,
                    e
                );

                // Mark server as failed
                mark_server_failed(&server_url).await;

                last_error = e;

                // Wait before retry (exponential backoff)
                if attempt < MAX_RETRIES {
                    let delay = std::time::Duration::from_millis(100 * (2_u64.pow(attempt - 1)));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    Err(format!(
        "All API servers failed after {} attempts. Last error: {}",
        MAX_RETRIES, last_error
    ))
}

/// Get connection pool statistics (for monitoring)
pub fn get_pool_stats() -> String {
    format!(
        "HTTP Client: max_idle={}, idle_timeout=90s, connect_timeout=5s, failover=enabled",
        10
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login_with_failover() {
        // This test requires at least one API server to be running
        match login("admin", "admin123").await {
            Ok(response) => {
                println!("✓ Login successful: {:?}", response);
                assert_eq!(response.status, "success");
            }
            Err(e) => {
                println!("✗ Login failed (expected if no servers running): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_failover_behavior() {
        use crate::api_server_config::get_server_stats;

        println!("Testing failover behavior...");

        // Attempt login (will try all servers)
        let _ = login("admin", "admin123").await;

        // Check server stats
        let stats = get_server_stats().await;
        println!("\nServer Status:");
        for (url, status) in stats {
            println!(
                "  {} - Healthy: {}, Failures: {}, Response: {:?}ms",
                url, status.is_healthy, status.consecutive_failures, status.response_time_ms
            );
        }
    }
}

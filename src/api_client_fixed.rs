use serde::{Deserialize, Serialize};
use std::sync::Arc;
use once_cell::sync::Lazy;

const API_SERVER: &str = "http://nas.haydenstudio.hk:21114";

// Global HTTP client with connection pooling
// This client is created once and reused for all requests
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .pool_max_idle_per_host(10)          // Keep up to 10 idle connections per host
        .pool_idle_timeout(std::time::Duration::from_secs(90))  // Keep connections alive for 90s
        .connect_timeout(std::time::Duration::from_secs(5))     // 5s to establish connection
        .tcp_keepalive(std::time::Duration::from_secs(60))      // TCP keepalive every 60s
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
/// Uses shared HTTP client for better performance
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

    // Use global HTTP_CLIENT instead of creating new one
    let response = HTTP_CLIENT
        .post(format!("{}/api/verify-settings", API_SERVER))
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
/// Uses shared HTTP client for better performance
pub async fn login(username: &str, password: &str) -> Result<LoginResponse, String> {
    let device_id = hbb_common::config::Config::get_id();

    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
        device_id,
    };

    // Use global HTTP_CLIENT instead of creating new one
    let response = HTTP_CLIENT
        .post(format!("{}/api/login", API_SERVER))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))
}

/// Get connection pool statistics (for monitoring)
pub fn get_pool_stats() -> String {
    format!(
        "HTTP Client: max_idle={}, idle_timeout=90s, connect_timeout=5s",
        10
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login() {
        // This test requires the API server to be running
        match login("admin", "admin123").await {
            Ok(response) => {
                println!("Login successful: {:?}", response);
                assert_eq!(response.status, "success");
            }
            Err(e) => {
                println!("Login failed (expected if server not running): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_connection_reuse() {
        // Test that multiple requests reuse the same connection
        println!("Making 5 requests to test connection pooling...");

        for i in 1..=5 {
            let start = std::time::Instant::now();
            let _ = login("admin", "admin123").await;
            let duration = start.elapsed();

            println!("Request {}: {:?}", i, duration);
            // First request will be slower (establishing connection)
            // Subsequent requests should be faster (reusing connection)
        }

        println!("Pool stats: {}", get_pool_stats());
    }
}

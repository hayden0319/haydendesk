use serde::{Deserialize, Serialize};

const API_SERVER: &str = "http://nas.haydenstudio.hk:21114";

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

pub async fn verify_settings_password(
    username: &str,
    password: &str,
) -> Result<VerifySettingsResponse, String> {
    let device_id = hbb_common::config::Config::get_id();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let request = VerifySettingsRequest {
        device_id,
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
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

pub async fn login(username: &str, password: &str) -> Result<LoginResponse, String> {
    let device_id = hbb_common::config::Config::get_id();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
        device_id,
    };

    let response = client
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
}

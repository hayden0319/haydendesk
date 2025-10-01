#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use librustdesk::{api_client, simple_permissions::PermissionConfig};
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, CustomMenuItem};

#[tauri::command]
fn get_device_id() -> String {
    hbb_common::config::Config::get_id()
}

#[tauri::command]
fn get_permissions() -> PermissionConfig {
    PermissionConfig::load()
}

#[tauri::command]
async fn verify_settings_password(username: String, password: String) -> Result<serde_json::Value, String> {
    match api_client::verify_settings_password(&username, &password).await {
        Ok(response) => {
            Ok(serde_json::json!({
                "valid": true,
                "username": response.username,
                "role": response.role,
                "can_modify_settings": response.can_modify_settings
            }))
        }
        Err(e) => Err(e)
    }
}

#[tauri::command]
async fn login_user(username: String, password: String) -> Result<serde_json::Value, String> {
    match api_client::login(&username, &password).await {
        Ok(response) => {
            if response.status == "success" {
                Ok(serde_json::json!({
                    "success": true,
                    "username": response.username,
                    "role": response.role,
                    "can_modify_settings": response.can_modify_settings,
                    "token": response.token
                }))
            } else {
                Err("Login failed".to_string())
            }
        }
        Err(e) => Err(e)
    }
}

#[tauri::command]
async fn set_permission(perm_type: String, enabled: bool, username: String, password: String) -> Result<String, String> {
    // Verify with API first
    api_client::verify_settings_password(&username, &password).await?;

    // If verified, update local config
    let mut config = PermissionConfig::load();

    match perm_type.as_str() {
        "mouse" => config.allow_mouse = enabled,
        "keyboard" => config.allow_keyboard = enabled,
        "clipboard" => config.allow_clipboard = enabled,
        "file" => config.allow_file_transfer = enabled,
        "audio" => config.allow_audio = enabled,
        _ => return Err("Unknown permission type".to_string()),
    }

    config.save();
    hbb_common::log::info!("Permission '{}' set to {} by user '{}'", perm_type, enabled, username);
    Ok("Permission updated".to_string())
}

#[tauri::command]
async fn enable_all_permissions(username: String, password: String) -> Result<String, String> {
    // Verify with API
    api_client::verify_settings_password(&username, &password).await?;

    let mut config = PermissionConfig::load();
    config.enable_all_permissions();
    hbb_common::log::info!("All permissions enabled by user '{}'", username);
    Ok("All permissions enabled".to_string())
}

#[tauri::command]
async fn disable_all_permissions(username: String, password: String) -> Result<String, String> {
    // Verify with API
    api_client::verify_settings_password(&username, &password).await?;

    let mut config = PermissionConfig::load();
    config.disable_all_permissions();
    hbb_common::log::info!("All permissions disabled by user '{}'", username);
    Ok("All permissions disabled".to_string())
}

fn main() {
    // Initialize logging
    hbb_common::log::info!("Starting FamilyDesk Tauri application");

    // Create system tray
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show Window"))
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide Window"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                "hide" => {
                    if let Some(window) = app.get_window("main") {
                        window.hide().unwrap();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            SystemTrayEvent::DoubleClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_device_id,
            get_permissions,
            verify_settings_password,
            login_user,
            set_permission,
            enable_all_permissions,
            disable_all_permissions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

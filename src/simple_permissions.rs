use hbb_common::config::LocalConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionConfig {
    // Simple toggles - no complex tracking
    pub allow_keyboard: bool,
    pub allow_mouse: bool,
    pub allow_clipboard: bool,
    pub allow_file_transfer: bool,
    pub allow_audio: bool,

    // Simple password
    pub fixed_password: String,

    // Simple device ID
    pub device_id: Option<String>,

    // Note: Master password removed - now using API server authentication
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self {
            allow_keyboard: false,
            allow_mouse: false,
            allow_clipboard: true,
            allow_file_transfer: false,
            allow_audio: true,
            fixed_password: "192.168.31.1".to_string(),
            device_id: None,
        }
    }
}

impl PermissionConfig {
    pub fn load() -> Self {
        if let Some(config_str) = LocalConfig::get_option("simple_permissions") {
            serde_json::from_str(&config_str).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string(self) {
            LocalConfig::set_option("simple_permissions".to_string(), json);
        }
    }

    pub fn enable_all_permissions(&mut self) {
        self.allow_keyboard = true;
        self.allow_mouse = true;
        self.allow_clipboard = true;
        self.allow_file_transfer = true;
        self.allow_audio = true;
        self.save();
    }

    pub fn disable_all_permissions(&mut self) {
        self.allow_keyboard = false;
        self.allow_mouse = false;
        self.allow_clipboard = false;
        self.allow_file_transfer = false;
        self.allow_audio = false;
        self.save();
    }

    pub fn apply_default_password(&self) {
        if !self.fixed_password.is_empty() {
            hbb_common::config::Config::set_password(&self.fixed_password);
        }
    }

    pub fn set_custom_device_id(&self) {
        if let Some(ref device_id) = self.device_id {
            hbb_common::config::Config::set_id(device_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PermissionConfig::default();
        assert_eq!(config.fixed_password, "192.168.31.1");
        assert!(!config.allow_keyboard);
        assert!(!config.allow_mouse);
        assert!(config.allow_clipboard);
    }

    #[test]
    fn test_config_serialization() {
        let config = PermissionConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PermissionConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.fixed_password, deserialized.fixed_password);
    }

    #[test]
    fn test_enable_all() {
        let mut config = PermissionConfig::default();
        config.enable_all_permissions();
        assert!(config.allow_keyboard);
        assert!(config.allow_mouse);
        assert!(config.allow_clipboard);
        assert!(config.allow_file_transfer);
        assert!(config.allow_audio);
    }

    #[test]
    fn test_disable_all() {
        let mut config = PermissionConfig::default();
        config.enable_all_permissions();
        config.disable_all_permissions();
        assert!(!config.allow_keyboard);
        assert!(!config.allow_mouse);
        assert!(!config.allow_clipboard);
        assert!(!config.allow_file_transfer);
        assert!(!config.allow_audio);
    }
}

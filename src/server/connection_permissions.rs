/// Permission enforcement module for FamilyDesk
/// This module integrates with simple_permissions to actually block/allow input
///
/// Add this to src/server/connection.rs at the top:
/// ```rust
/// #[cfg(feature = "simple_permissions")]
/// use crate::simple_permissions::PermissionConfig;
/// ```

#[cfg(feature = "simple_permissions")]
use crate::simple_permissions::PermissionConfig;
use hbb_common::log;

/// Check if mouse input is allowed
/// Returns true if allowed, false if blocked
#[cfg(feature = "simple_permissions")]
pub fn check_mouse_permission() -> bool {
    let config = PermissionConfig::load();
    if !config.allow_mouse {
        log::warn!("🚫 Mouse input blocked by FamilyDesk permissions");
        return false;
    }
    true
}

/// Check if keyboard input is allowed
/// Returns true if allowed, false if blocked
#[cfg(feature = "simple_permissions")]
pub fn check_keyboard_permission() -> bool {
    let config = PermissionConfig::load();
    if !config.allow_keyboard {
        log::warn!("🚫 Keyboard input blocked by FamilyDesk permissions");
        return false;
    }
    true
}

/// Check if clipboard is allowed
/// Returns true if allowed, false if blocked
#[cfg(feature = "simple_permissions")]
pub fn check_clipboard_permission() -> bool {
    let config = PermissionConfig::load();
    if !config.allow_clipboard {
        log::warn!("🚫 Clipboard blocked by FamilyDesk permissions");
        return false;
    }
    true
}

/// Check if file transfer is allowed
/// Returns true if allowed, false if blocked
#[cfg(feature = "simple_permissions")]
pub fn check_file_transfer_permission() -> bool {
    let config = PermissionConfig::load();
    if !config.allow_file_transfer {
        log::warn!("🚫 File transfer blocked by FamilyDesk permissions");
        return false;
    }
    true
}

/// Check if audio is allowed
/// Returns true if allowed, false if blocked
#[cfg(feature = "simple_permissions")]
pub fn check_audio_permission() -> bool {
    let config = PermissionConfig::load();
    if !config.allow_audio {
        log::warn!("🚫 Audio blocked by FamilyDesk permissions");
        return false;
    }
    true
}

// If feature not enabled, always allow (no-op)
#[cfg(not(feature = "simple_permissions"))]
pub fn check_mouse_permission() -> bool { true }

#[cfg(not(feature = "simple_permissions"))]
pub fn check_keyboard_permission() -> bool { true }

#[cfg(not(feature = "simple_permissions"))]
pub fn check_clipboard_permission() -> bool { true }

#[cfg(not(feature = "simple_permissions"))]
pub fn check_file_transfer_permission() -> bool { true }

#[cfg(not(feature = "simple_permissions"))]
pub fn check_audio_permission() -> bool { true }

# FamilyDesk - API-Based Remote Support System

**Version:** 1.0.0
**Based on:** RustDesk 1.4.2
**API Server:** http://nas.haydenstudio.hk:21114

## Overview

FamilyDesk is a simplified, API-authenticated remote desktop solution built on RustDesk, designed for:
- **Family Support:** Help elderly relatives with their computers
- **School IT:** Manage student computers in educational environments
- **Secure Access Control:** Centralized API-based authentication

## Key Features

✅ **API-Based Authentication** - All permission changes require server authentication
✅ **Role-Based Access** - Admin, Family, Student accounts with different permissions
✅ **Device-Level Control** - Restrict accounts to specific devices
✅ **Simple Permissions** - Toggle keyboard, mouse, clipboard, file transfer, audio
✅ **Pre-configured Relay** - Hardcoded server: `nas.haydenstudio.hk`
✅ **Tauri UI** - Modern, accessible interface for elderly users

---

## Quick Start

### 1. Start API Server

```bash
cd /Users/hayden/Downloads/rustdesk2-master
cargo run --bin api_server --features api_server
```

**Default Credentials:**
- Username: `admin`
- Password: `admin123`

⚠️ **IMPORTANT:** Change the admin password immediately!

### 2. Build FamilyDesk Client

```bash
# Build with FamilyDesk features
cargo build --release --features family_desk

# Or build Tauri UI
cd tauri-src
cargo tauri build
```

### 3. Deploy to Client Machine

```bash
# Copy executable to target machine
# Windows: familydesk.exe
# macOS: familydesk.app
# Linux: familydesk

# Run with custom device ID
./familydesk --device-id "GRANDMA_PC"
```

---

## API Server Setup

### Create Accounts

Use the management script:

```bash
./manage_accounts.sh
```

Or manually with curl:

```bash
# Create family member account
curl -X POST "http://nas.haydenstudio.hk:21114/api/create-account" \
  -H "Content-Type: application/json" \
  -d '{
    "admin_password": "admin123",
    "new_username": "john_family",
    "new_password": "family2025",
    "role": "family",
    "can_modify_settings": true,
    "device_ids": ["GRANDMA_PC", "MOM_PC"]
  }'
```

### Account Types

**Admin Account:**
```json
{
  "username": "admin",
  "role": "admin",
  "can_modify_settings": true,
  "device_ids": ["*"]  // All devices
}
```

**Family Account:**
```json
{
  "username": "john_family",
  "role": "family",
  "can_modify_settings": true,
  "device_ids": ["GRANDMA_PC", "DAD_PC"]
}
```

**Student Account:**
```json
{
  "username": "student01",
  "role": "student",
  "can_modify_settings": false,
  "device_ids": ["SCHOOL_PC_01"]
}
```

---

## Usage

### On Controlled Computer (Elderly/Student)

1. Launch FamilyDesk
2. Share the **Device ID** displayed on screen
3. Toggle permissions as needed
4. **All changes require API authentication**

### Permission Toggles

- 🖱️ **Mouse Control** - Allow remote mouse input
- ⌨️ **Keyboard Control** - Allow remote keyboard input
- 📋 **Clipboard** - Share clipboard between computers
- 📁 **File Transfer** - Allow file uploads/downloads
- 🔊 **Audio** - Stream audio from controlled computer

### Authenticating Changes

When you toggle any permission:

1. Enter your **username** (e.g., `john_family`)
2. Enter your **password**
3. System verifies with API server
4. If approved, permission is updated

---

## Architecture

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│  Control End    │◄───────►│  Relay Server    │◄───────►│  Controlled End │
│  (Family/IT)    │         │ nas.haydenstudio │         │  (Elderly/Lab)  │
└─────────────────┘         └──────────────────┘         └─────────────────┘
                                     ▲                              ▲
                                     │                              │
                                     └──────────────────────────────┘
                                              ▼
                                    ┌──────────────────┐
                                    │   API Server     │
                                    │  :21114          │
                                    │  - Auth          │
                                    │  - Permissions   │
                                    │  - Account Mgmt  │
                                    └──────────────────┘
```

### Components

1. **API Server** (`src/api_server/mod.rs`)
   - Account management
   - JWT token generation
   - Permission verification

2. **API Client** (`src/api_client.rs`)
   - Authenticates with server
   - Verifies permissions

3. **Simple Permissions** (`src/simple_permissions.rs`)
   - Local permission storage
   - Toggle management

4. **Tauri UI** (`tauri-src/`)
   - User interface
   - Permission controls

---

## Configuration

### Fixed Password

Default connection password: `192.168.31.1`

Change in `src/simple_permissions.rs`:

```rust
fixed_password: "YOUR_NEW_PASSWORD".to_string(),
```

### Relay Server

Pre-configured in `libs/hbb_common/src/config.rs`:

```rust
pub const RENDEZVOUS_SERVERS: &[&str] = &["nas.haydenstudio.hk"];
pub const RS_PUB_KEY: &str = "iQX+6aUbS40CL9jJElm4jSMs8aKzePqMsFH1HACDI5E=";
```

### API Server URL

Change in `src/api_client.rs`:

```rust
const API_SERVER: &str = "http://nas.haydenstudio.hk:21114";
```

---

## Deployment

### School Lab Deployment

**Batch Script (Windows):**

```batch
@echo off
REM Deploy FamilyDesk to School Computers

SET COMPUTER_ID=%COMPUTERNAME%

REM Install to Program Files
xcopy /E /I /Y .\familydesk "C:\Program Files\FamilyDesk\"

REM Set device ID
"C:\Program Files\FamilyDesk\familydesk.exe" --device-id "%COMPUTER_ID%"

REM Create startup entry
REG ADD "HKLM\Software\Microsoft\Windows\CurrentVersion\Run" /v FamilyDesk /t REG_SZ /d "C:\Program Files\FamilyDesk\familydesk.exe" /f

echo Deployed to %COMPUTER_ID%
pause
```

### Family Computer Setup

1. Install FamilyDesk
2. Set custom device ID: `./familydesk --device-id "GRANDMA_PC"`
3. Create family account on API server
4. Share device ID with family member

---

## API Endpoints

### `/api/login`

**Request:**
```json
{
  "username": "john_family",
  "password": "family2025",
  "device_id": "GRANDMA_PC"
}
```

**Response:**
```json
{
  "status": "success",
  "token": "eyJhbGc...",
  "username": "john_family",
  "role": "family",
  "can_modify_settings": true
}
```

### `/api/verify-settings`

**Request:**
```json
{
  "device_id": "GRANDMA_PC",
  "username": "john_family",
  "password": "family2025"
}
```

**Response:**
```json
{
  "status": "success",
  "can_modify_settings": true,
  "username": "john_family",
  "role": "family"
}
```

### `/api/create-account`

**Request:**
```json
{
  "admin_password": "admin123",
  "new_username": "new_user",
  "new_password": "password",
  "role": "family",
  "can_modify_settings": true,
  "device_ids": ["DEVICE_01"]
}
```

### `/api/list-accounts`

**Request:**
```json
{
  "admin_password": "admin123"
}
```

---

## Security

### Authentication Flow

1. User attempts to change permission
2. UI prompts for username/password
3. Client sends credentials to API server
4. API server verifies:
   - Username exists
   - Password is correct (Argon2 hash)
   - User has access to this device
   - User can modify settings
5. If verified, permission is updated locally

### Password Security

- **Argon2** hashing for all passwords
- **JWT tokens** for session management (7-day expiration)
- **HTTPS recommended** for production (configure in API server)

### Network Security

- API server should be behind firewall
- Use VPN for remote API access
- Enable HTTPS with valid certificates

---

## Troubleshooting

### API Server Not Responding

```bash
# Check if server is running
curl http://nas.haydenstudio.hk:21114/health

# Should return: OK
```

### Authentication Fails

```bash
# Test login directly
curl -X POST "http://nas.haydenstudio.hk:21114/api/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123","device_id":"TEST"}'
```

### Permissions Not Saving

Check logs:
```bash
# On client machine
tail -f ~/.config/RustDesk/RustDesk.log
```

---

## Development

### Build Components

```bash
# API Server only
cargo build --features api_server

# Client with simple permissions
cargo build --features simple_permissions

# Full FamilyDesk (all features)
cargo build --features family_desk

# Tauri UI
cd tauri-src && cargo tauri build
```

### Run Tests

```bash
# Test permissions module
cargo test --features simple_permissions

# Test API client (requires server running)
cargo test --features simple_permissions -- --test-threads=1
```

---

## License

Based on RustDesk (GNU General Public License v3.0)

---

## Support

For issues or questions:
- Check API server logs
- Verify network connectivity
- Ensure accounts have correct permissions
- Review device ID configuration

**API Server:** http://nas.haydenstudio.hk:21114
**Health Check:** http://nas.haydenstudio.hk:21114/health

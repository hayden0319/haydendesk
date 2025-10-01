# FamilyDesk Code Review & Improvements Needed

**Date:** 2025-02-10
**Reviewer:** Code Analysis
**Status:** 🔴 Multiple Critical Issues Found

---

## 🚨 CRITICAL ISSUES (Must Fix Before Production)

### 1. ❌ **Hardcoded JWT Secret**
**File:** `src/api_server/mod.rs:45`

```rust
const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";
```

**Problem:**
- Secret key is hardcoded in source code
- Anyone with source code can forge JWT tokens
- Cannot be changed without recompiling

**Solution:**
```rust
// Load from environment variable
use std::env;

lazy_static::lazy_static! {
    static ref JWT_SECRET: Vec<u8> = {
        env::var("JWT_SECRET")
            .expect("JWT_SECRET environment variable must be set")
            .into_bytes()
    };
}
```

**Priority:** 🔴 CRITICAL

---

### 2. ❌ **Insecure HTTP (No HTTPS)**
**File:** `src/api_client.rs:3`

```rust
const API_SERVER: &str = "http://nas.haydenstudio.hk:21114";
```

**Problem:**
- Passwords sent in plain text over network
- Susceptible to man-in-the-middle attacks
- Anyone on network can intercept credentials

**Solution:**
```rust
const API_SERVER: &str = "https://nas.haydenstudio.hk:21114";

// In api_server/mod.rs, add TLS:
use actix_web::{HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
builder.set_private_key_file("key.pem", SslFiletype::PEM)?;
builder.set_certificate_chain_file("cert.pem")?;

HttpServer::new(|| { /* ... */ })
    .bind_openssl("0.0.0.0:21114", builder)?
    .run()
    .await
```

**Priority:** 🔴 CRITICAL

---

### 3. ❌ **In-Memory Account Storage**
**File:** `src/api_server/mod.rs:48-58`

```rust
lazy_static::lazy_static! {
    static ref ACCOUNTS: Mutex<HashMap<String, Account>> = {
        // All accounts lost on restart!
    };
}
```

**Problems:**
- All accounts lost when server restarts
- No persistence across reboots
- No backup/recovery
- Cannot scale to multiple servers

**Solution:**
```rust
// Add to Cargo.toml
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }

// Use PostgreSQL or SQLite
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub can_modify_settings: bool,
    pub device_ids: sqlx::types::Json<Vec<String>>,
}

pub async fn create_account(pool: &PgPool, account: Account) -> Result<()> {
    sqlx::query!(
        "INSERT INTO accounts (username, password_hash, role, can_modify_settings, device_ids)
         VALUES ($1, $2, $3, $4, $5)",
        account.username,
        account.password_hash,
        account.role,
        account.can_modify_settings,
        sqlx::types::Json(account.device_ids) as _
    )
    .execute(pool)
    .await?;
    Ok(())
}
```

**Priority:** 🔴 CRITICAL

---

### 4. ❌ **Default Admin Password Never Expires**
**File:** `src/api_server/mod.rs:51-56`

```rust
m.insert("admin".to_string(), Account {
    username: "admin".to_string(),
    password_hash: hash_password("admin123"), // NEVER CHANGES!
    // ...
});
```

**Problem:**
- Default password `admin123` is public knowledge
- No forced password change on first login
- Security breach waiting to happen

**Solution:**
```rust
// Add password change requirement
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
    pub must_change_password: bool, // NEW
    pub last_password_change: Option<SystemTime>, // NEW
    // ...
}

// Force change on first login
if account.must_change_password {
    return HttpResponse::Forbidden().json(json!({
        "status": "password_change_required",
        "message": "You must change your password before continuing"
    }));
}
```

**Priority:** 🔴 CRITICAL

---

## ⚠️ MAJOR ISSUES (Should Fix Soon)

### 5. ⚠️ **No Rate Limiting**
**File:** `src/api_server/mod.rs` (missing)

**Problem:**
- Unlimited login attempts possible
- Brute force attacks can succeed
- No protection against credential stuffing

**Solution:**
```rust
// Add to Cargo.toml
actix-governor = "0.5"

use actix_governor::{Governor, GovernorConfigBuilder};

let governor_conf = GovernorConfigBuilder::default()
    .per_second(2)  // 2 requests per second
    .burst_size(5)  // Allow 5 requests burst
    .finish()
    .unwrap();

App::new()
    .wrap(Governor::new(&governor_conf))
    .route("/api/login", web::post().to(login))
```

**Priority:** 🟠 MAJOR

---

### 6. ⚠️ **Password Sent Every Request**
**File:** `src/api_client.rs:20-46`

**Problem:**
- Password sent on every permission change
- Increases exposure window
- No session management

**Solution:**
```rust
// Login once, store JWT token
pub async fn login_and_store_token(username: &str, password: &str) -> Result<String> {
    let response = login(username, password).await?;
    if let Some(token) = response.token {
        // Store token securely
        LocalConfig::set_option("auth_token".to_string(), token.clone());
        Ok(token)
    } else {
        Err("No token received".to_string())
    }
}

// Use token for subsequent requests
pub async fn verify_with_token(token: &str) -> Result<VerifySettingsResponse> {
    // Just send token, not password
}
```

**Priority:** 🟠 MAJOR

---

### 7. ⚠️ **No Audit Logging**
**File:** All API endpoints (missing)

**Problem:**
- No record of who changed what
- Cannot investigate security incidents
- No compliance trail

**Solution:**
```rust
#[derive(Serialize)]
struct AuditLog {
    timestamp: SystemTime,
    username: String,
    action: String,
    device_id: String,
    ip_address: String,
    success: bool,
}

async fn log_audit(log: AuditLog) {
    // Write to database or file
    let json = serde_json::to_string(&log).unwrap();
    tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/familydesk/audit.log")
        .await?
        .write_all(format!("{}\n", json).as_bytes())
        .await?;
}

// In each endpoint:
log_audit(AuditLog {
    timestamp: SystemTime::now(),
    username: req.username.clone(),
    action: "login".to_string(),
    device_id: req.device_id.clone(),
    ip_address: peer_addr.to_string(),
    success: true,
}).await;
```

**Priority:** 🟠 MAJOR

---

### 8. ⚠️ **No Input Validation**
**File:** Multiple endpoints

**Problem:**
- Username could be empty or contain special characters
- Device IDs not validated
- Potential SQL injection if database added

**Solution:**
```rust
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    #[validate(regex = "USERNAME_REGEX")]
    username: String,

    #[validate(length(min = 8))]
    password: String,

    #[validate(length(min = 1, max = 100))]
    device_id: String,
}

lazy_static::lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }
    // ...
}
```

**Priority:** 🟠 MAJOR

---

## 🟡 MODERATE ISSUES (Nice to Have)

### 9. 🟡 **No Password Complexity Requirements**
**File:** `src/api_server/mod.rs:hash_password()`

**Problem:**
- Accepts weak passwords like "123456"
- No minimum complexity enforcement

**Solution:**
```rust
fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain at least one lowercase letter".to_string());
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number".to_string());
    }
    Ok(())
}
```

**Priority:** 🟡 MODERATE

---

### 10. 🟡 **No Token Refresh Mechanism**
**File:** `src/api_server/mod.rs:login()`

**Problem:**
- JWT expires after 7 days
- User must re-login completely
- No graceful token renewal

**Solution:**
```rust
pub async fn refresh_token(req: web::Json<RefreshTokenRequest>) -> impl Responder {
    match decode::<Claims>(
        &req.refresh_token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            // Issue new token
            let new_token = generate_token(token_data.claims)?;
            HttpResponse::Ok().json(json!({"token": new_token}))
        }
        Err(_) => HttpResponse::Unauthorized().json(json!({"error": "Invalid refresh token"}))
    }
}
```

**Priority:** 🟡 MODERATE

---

### 11. 🟡 **No Connection Pooling**
**File:** `src/api_client.rs:26-29`

**Problem:**
- Creates new HTTP client for every request
- Wastes resources
- Slower performance

**Solution:**
```rust
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to create HTTP client");
}

pub async fn verify_settings_password(
    username: &str,
    password: &str,
) -> Result<VerifySettingsResponse, String> {
    // Use shared client
    let response = HTTP_CLIENT
        .post(format!("{}/api/verify-settings", API_SERVER))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    // ...
}
```

**Priority:** 🟡 MODERATE

---

### 12. 🟡 **Cargo.toml Dependency Issues**
**File:** `Cargo.toml:93`

**Problem:**
```toml
reqwest = { version = "0.11", features = ["json"], optional = true }
```

- `reqwest` already declared at line 169-173 with different features
- Potential version conflict
- Won't compile on Linux (different reqwest config)

**Solution:**
```toml
# Remove duplicate, use existing one
# Line 169-173 already has reqwest
# Just add "json" feature to existing declaration

[target.'cfg(any(target_os = "macos", target_os = "windows"))'.dependencies]
reqwest = {
    git = "https://github.com/rustdesk-org/reqwest",
    features = ["blocking", "socks", "json", "native-tls", "gzip"],
    default-features=false
}
```

**Priority:** 🟡 MODERATE (will cause compile error)

---

## 🔵 MINOR ISSUES (Polish)

### 13. 🔵 **No Graceful Shutdown**
**File:** `src/api_server/mod.rs:start_api_server()`

**Problem:**
- Server stops immediately on Ctrl+C
- In-flight requests may be lost
- No cleanup of resources

**Solution:**
```rust
use actix_web::middleware::Logger;
use tokio::signal;

#[actix_web::main]
pub async fn start_api_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // ...
    })
    .bind(("0.0.0.0", 21114))?
    .run();

    let server_handle = server.handle();

    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("Shutting down gracefully...");
        server_handle.stop(true).await;
    });

    server.await
}
```

**Priority:** 🔵 MINOR

---

### 14. 🔵 **Tauri UI Has No Loading States**
**File:** `tauri-src/index.html`

**Problem:**
- No loading indicator during API calls
- User doesn't know if request is processing
- Poor UX for slow networks

**Solution:**
```javascript
async function submitPassword() {
    const submitBtn = document.querySelector('.btn-enable');
    submitBtn.disabled = true;
    submitBtn.textContent = 'Authenticating...';

    try {
        // ... existing code ...
    } finally {
        submitBtn.disabled = false;
        submitBtn.textContent = 'Submit';
    }
}
```

**Priority:** 🔵 MINOR

---

### 15. 🔵 **No Error Code Standardization**
**File:** All API responses

**Problem:**
- Error messages are inconsistent strings
- Client must parse error text
- Difficult to handle different error types

**Solution:**
```rust
#[derive(Serialize)]
struct ApiError {
    code: String,
    message: String,
    details: Option<serde_json::Value>,
}

// Standardized error codes
const ERR_INVALID_CREDENTIALS: &str = "ERR_INVALID_CREDENTIALS";
const ERR_NO_ACCESS: &str = "ERR_NO_ACCESS";
const ERR_ACCOUNT_EXISTS: &str = "ERR_ACCOUNT_EXISTS";

HttpResponse::Unauthorized().json(ApiError {
    code: ERR_INVALID_CREDENTIALS.to_string(),
    message: "Invalid username or password".to_string(),
    details: None,
})
```

**Priority:** 🔵 MINOR

---

## 🏗️ ARCHITECTURAL CONCERNS

### 16. **Tauri Not Properly Integrated with RustDesk**
**Files:** `tauri-src/*`

**Problem:**
- Tauri UI is separate from RustDesk main application
- `tauri-src/main.rs` imports `librustdesk` but main RustDesk uses Sciter/Flutter
- No integration with existing `src/server/connection.rs` for actual permission enforcement
- Permissions checked in UI but not enforced in connection handler

**Reality Check:**
The Tauri UI can show/hide permissions, but **RustDesk connection handler doesn't actually check these permissions!**

**What's Missing:**
```rust
// In src/server/connection.rs (NOT IMPLEMENTED)
impl Connection {
    fn handle_mouse_event(&mut self, me: MouseEvent) {
        // THIS CHECK IS MISSING:
        let config = PermissionConfig::load();
        if !config.allow_mouse {
            log::warn!("Mouse input blocked by permissions");
            return; // Block the input
        }
        // ... existing code ...
    }
}
```

**Solution Needed:**
1. Modify `src/server/connection.rs` to check permissions before processing input
2. Or integrate permission checks into message routing
3. Or use RustDesk's existing permission system instead of new one

**Priority:** 🔴 CRITICAL - Current system doesn't actually enforce permissions!

---

### 17. **Permission Module Not Connected to RustDesk Core**
**File:** `src/simple_permissions.rs`

**Problem:**
- Standalone module with no hooks into RustDesk
- Saves to LocalConfig but RustDesk doesn't read from it
- No integration with existing RustDesk permission system

**RustDesk Already Has Permissions:**
- In `src/client.rs` and `src/server/connection.rs`
- Uses `PermissionInfo` struct
- Connected to UI via message passing

**Solution:**
Need to either:
1. **Option A:** Hook into existing system
```rust
// Use RustDesk's existing permission system
use crate::server::PermissionInfo;

pub fn apply_permissions_to_connection(conn: &mut Connection, config: &PermissionConfig) {
    let mut perms = PermissionInfo::default();
    perms.keyboard = config.allow_keyboard;
    perms.mouse = config.allow_mouse;
    perms.clipboard = config.allow_clipboard;
    // ...
    conn.set_permissions(perms);
}
```

2. **Option B:** Modify RustDesk to use your new system (major refactoring)

**Priority:** 🔴 CRITICAL

---

### 18. **No Binary for API Server**
**File:** `Cargo.toml` (missing)

**Problem:**
```toml
[[bin]]
name = "naming"
path = "src/naming.rs"

[[bin]]
name = "service"
path = "src/service.rs"

# API SERVER BINARY IS MISSING!
```

- Cannot run API server as standalone binary
- `cargo run --features api_server` will run main RustDesk, not API server

**Solution:**
```toml
[[bin]]
name = "api_server"
path = "src/api_server/main.rs"
```

Create `src/api_server/main.rs`:
```rust
#[tokio::main]
async fn main() {
    librustdesk::api_server::start_api_server()
        .await
        .expect("Failed to start API server");
}
```

**Priority:** 🔴 CRITICAL - API server cannot be started!

---

## 📊 SUMMARY

### Critical Issues (Must Fix): 8
1. ❌ Hardcoded JWT secret
2. ❌ No HTTPS
3. ❌ In-memory storage (data loss)
4. ❌ Default password never expires
5. ❌ Permissions not enforced in connection handler
6. ❌ Permission module not integrated
7. ❌ No API server binary
8. ❌ Duplicate reqwest dependency

### Major Issues (Should Fix): 4
5. ⚠️ No rate limiting
6. ⚠️ Password sent every request
7. ⚠️ No audit logging
8. ⚠️ No input validation

### Moderate Issues (Nice to Have): 4
9. 🟡 No password complexity
10. 🟡 No token refresh
11. 🟡 No connection pooling
12. 🟡 Cargo dependency conflict

### Minor Issues (Polish): 3
13. 🔵 No graceful shutdown
14. 🔵 No loading states
15. 🔵 No error codes

---

## ✅ WHAT WORKS WELL

1. ✅ **Good Architecture Ideas**
   - API-based authentication is sound
   - Role-based access makes sense
   - Device-level control is useful

2. ✅ **Clean Code Structure**
   - Well-organized modules
   - Clear separation of concerns
   - Good use of Rust idioms

3. ✅ **Good Documentation**
   - Comprehensive README
   - Quick start guide
   - Clear examples

4. ✅ **Security Basics**
   - Argon2 password hashing (good!)
   - JWT tokens (properly structured)
   - API design is secure-by-default

---

## 🎯 IMMEDIATE ACTION ITEMS

### Before You Can Test:

1. **Fix API Server Binary** (5 minutes)
```bash
# Create src/api_server/main.rs
# Add [[bin]] to Cargo.toml
```

2. **Fix Reqwest Conflict** (2 minutes)
```bash
# Remove duplicate reqwest from Cargo.toml line 93
```

3. **Integrate Permissions with Connection Handler** (30 minutes)
```rust
# Modify src/server/connection.rs
# Add permission checks before processing input
```

### Before Production:

4. **Enable HTTPS** (1 hour)
5. **Add Database Storage** (2 hours)
6. **Change JWT Secret** (5 minutes)
7. **Add Rate Limiting** (30 minutes)
8. **Force Admin Password Change** (30 minutes)

---

## 🤔 ARCHITECTURAL RECOMMENDATION

**Current Approach:**
New permission system separate from RustDesk core

**Better Approach:**
Integrate with existing RustDesk permission system

**Why:**
- RustDesk already has permissions (`PermissionInfo`)
- Your API can return RustDesk's permission format
- Less refactoring needed
- Works with existing UI (Sciter/Flutter)
- Your Tauri UI becomes optional admin interface

**Suggested Flow:**
```
1. API Server stores permissions
2. Client fetches permissions from API on startup
3. Client applies permissions to RustDesk's PermissionInfo
4. RustDesk enforces permissions (existing code)
5. Tauri UI is admin panel, not required for operation
```

---

## 📝 FINAL VERDICT

**Can it work?** Yes, with significant fixes
**Is it production-ready?** No
**Estimated time to production:** 8-16 hours of work

**Biggest Gaps:**
1. Permissions not actually enforced
2. API server can't start (no binary)
3. No HTTPS (passwords in plain text)
4. No persistent storage (data loss on restart)

**Recommendation:**
Fix critical issues (#1-8) before any deployment, even for testing.

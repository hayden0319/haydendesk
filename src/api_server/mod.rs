use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub device_id: String,
    pub family_member: String,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
    pub role: String, // "admin", "family", "student"
    pub can_modify_settings: bool,
    pub device_ids: Vec<String>, // Devices this account can access
}

#[derive(Deserialize)]
pub struct DeviceAuth {
    device_id: String,
    auth_token: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
    device_id: String,
}

#[derive(Deserialize)]
pub struct VerifySettingsRequest {
    device_id: String,
    username: String,
    password: String,
}

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";

// In-memory account storage (replace with database in production)
lazy_static::lazy_static! {
    static ref ACCOUNTS: Mutex<HashMap<String, Account>> = {
        let mut m = HashMap::new();
        // Default admin account
        m.insert("admin".to_string(), Account {
            username: "admin".to_string(),
            password_hash: hash_password("admin123"), // Change this!
            role: "admin".to_string(),
            can_modify_settings: true,
            device_ids: vec!["*".to_string()], // All devices
        });
        Mutex::new(m)
    };
}

fn hash_password(password: &str) -> String {
    use argon2::{Argon2, PasswordHasher, password_hash::SaltString, password_hash::rand_core::OsRng};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        return Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
    }
    false
}

// Login and get JWT token
pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    let accounts = ACCOUNTS.lock().unwrap();

    if let Some(account) = accounts.get(&req.username) {
        if verify_password(&req.password, &account.password_hash) {
            // Check if account has access to this device
            if account.device_ids.contains(&"*".to_string())
                || account.device_ids.contains(&req.device_id) {

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let claims = Claims {
                    device_id: req.device_id.clone(),
                    family_member: account.username.clone(),
                    exp: now + (86400 * 7), // 7 days
                    iat: now,
                };

                match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
                    Ok(token) => {
                        return HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "token": token,
                            "username": account.username,
                            "role": account.role,
                            "can_modify_settings": account.can_modify_settings
                        }));
                    }
                    Err(_) => {}
                }
            } else {
                return HttpResponse::Forbidden().json(serde_json::json!({
                    "status": "error",
                    "message": "No access to this device"
                }));
            }
        }
    }

    HttpResponse::Unauthorized().json(serde_json::json!({
        "status": "error",
        "message": "Invalid username or password"
    }))
}

// Verify if user can modify settings
pub async fn verify_settings_access(req: web::Json<VerifySettingsRequest>) -> impl Responder {
    let accounts = ACCOUNTS.lock().unwrap();

    if let Some(account) = accounts.get(&req.username) {
        if verify_password(&req.password, &account.password_hash) {
            // Check device access
            if account.device_ids.contains(&"*".to_string())
                || account.device_ids.contains(&req.device_id) {

                return HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "can_modify_settings": account.can_modify_settings,
                    "username": account.username,
                    "role": account.role
                }));
            }
        }
    }

    HttpResponse::Unauthorized().json(serde_json::json!({
        "status": "error",
        "message": "Invalid credentials or no access"
    }))
}

// Create new account (admin only)
#[derive(Deserialize)]
pub struct CreateAccountRequest {
    admin_password: String,
    new_username: String,
    new_password: String,
    role: String,
    can_modify_settings: bool,
    device_ids: Vec<String>,
}

pub async fn create_account(req: web::Json<CreateAccountRequest>) -> impl Responder {
    let mut accounts = ACCOUNTS.lock().unwrap();

    // Verify admin credentials
    if let Some(admin) = accounts.get("admin") {
        if !verify_password(&req.admin_password, &admin.password_hash) {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "status": "error",
                "message": "Invalid admin password"
            }));
        }
    } else {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Admin account not found"
        }));
    }

    // Check if username already exists
    if accounts.contains_key(&req.new_username) {
        return HttpResponse::Conflict().json(serde_json::json!({
            "status": "error",
            "message": "Username already exists"
        }));
    }

    // Create new account
    let new_account = Account {
        username: req.new_username.clone(),
        password_hash: hash_password(&req.new_password),
        role: req.role.clone(),
        can_modify_settings: req.can_modify_settings,
        device_ids: req.device_ids.clone(),
    };

    accounts.insert(req.new_username.clone(), new_account);

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Account created successfully",
        "username": req.new_username
    }))
}

// List all accounts (admin only)
#[derive(Deserialize)]
pub struct AdminRequest {
    admin_password: String,
}

pub async fn list_accounts(req: web::Json<AdminRequest>) -> impl Responder {
    let accounts = ACCOUNTS.lock().unwrap();

    // Verify admin
    if let Some(admin) = accounts.get("admin") {
        if !verify_password(&req.admin_password, &admin.password_hash) {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "status": "error",
                "message": "Invalid admin password"
            }));
        }
    }

    let account_list: Vec<_> = accounts.values()
        .map(|acc| serde_json::json!({
            "username": acc.username,
            "role": acc.role,
            "can_modify_settings": acc.can_modify_settings,
            "device_ids": acc.device_ids
        }))
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "accounts": account_list
    }))
}

pub async fn verify_device(auth: web::Json<DeviceAuth>) -> impl Responder {
    match decode::<Claims>(
        &auth.auth_token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            if token_data.claims.device_id == auth.device_id {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                if token_data.claims.exp > now {
                    return HttpResponse::Ok().json(serde_json::json!({
                        "status": "valid",
                        "family_member": token_data.claims.family_member
                    }));
                }
            }
            HttpResponse::Unauthorized().json(serde_json::json!({"status": "expired"}))
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({"status": "invalid"})),
    }
}

pub async fn generate_token(device: web::Json<serde_json::Value>) -> impl Responder {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        device_id: device["device_id"].as_str().unwrap_or("").to_string(),
        family_member: device["family_member"].as_str().unwrap_or("").to_string(),
        exp: now + (86400 * 365),
        iat: now,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({"token": token})),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
pub async fn start_api_server() -> std::io::Result<()> {
    println!("===========================================");
    println!("API Server starting on http://nas.haydenstudio.hk:21114");
    println!("Default admin credentials:");
    println!("  Username: admin");
    println!("  Password: admin123");
    println!("===========================================");
    println!("⚠️  IMPORTANT: Change the admin password immediately!");
    println!("===========================================");

    HttpServer::new(|| {
        App::new()
            .route("/api/login", web::post().to(login))
            .route("/api/verify-settings", web::post().to(verify_settings_access))
            .route("/api/create-account", web::post().to(create_account))
            .route("/api/list-accounts", web::post().to(list_accounts))
            .route("/api/verify", web::post().to(verify_device))
            .route("/api/generate", web::post().to(generate_token))
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
    })
    .bind(("0.0.0.0", 21114))?
    .run()
    .await
}

# FamilyDesk Quick Start Guide

## 🚀 Get Started in 5 Minutes

### Step 1: Start the API Server (on your server)

```bash
cd /Users/hayden/Downloads/rustdesk2-master
cargo run --bin api_server --features api_server
```

You should see:
```
===========================================
API Server starting on http://nas.haydenstudio.hk:21114
Default admin credentials:
  Username: admin
  Password: admin123
===========================================
```

**Keep this terminal open!**

---

### Step 2: Create Accounts

Open a new terminal and run:

```bash
./manage_accounts.sh
```

Choose option `1` to create an account:

```
Choose option (1-4): 1
Admin password: admin123
New username: john
New password: mypassword
Role (admin/family/student): family
Can modify settings? (true/false): true
Device IDs: GRANDMA_PC
```

---

### Step 3: Build FamilyDesk Client

```bash
cargo build --release --features family_desk
```

The executable will be at: `target/release/rustdesk`

---

### Step 4: Test It

**On the controlled computer (e.g., grandma's PC):**

```bash
cd target/release
./rustdesk --device-id "GRANDMA_PC"
```

The Tauri UI will open showing:
- Device ID: `GRANDMA_PC`
- Permission toggles (all disabled by default)

**Try changing a permission:**
1. Click the "Mouse Control" toggle
2. Enter username: `john`
3. Enter password: `mypassword`
4. Permission is enabled!

---

## 📝 Example Scenarios

### Scenario 1: Help Grandma

**Setup:**
```bash
# Create account for you
curl -X POST "http://nas.haydenstudio.hk:21114/api/create-account" \
  -H "Content-Type: application/json" \
  -d '{
    "admin_password": "admin123",
    "new_username": "john",
    "new_password": "family2025",
    "role": "family",
    "can_modify_settings": true,
    "device_ids": ["GRANDMA_PC"]
  }'
```

**On grandma's computer:**
```bash
./rustdesk --device-id "GRANDMA_PC"
```

**When you need to help:**
1. Call grandma and ask for the Device ID (shown on screen)
2. Connect using RustDesk with that ID
3. Grandma toggles permissions when you ask
4. You enter username/password to authenticate
5. Help complete, grandma disables permissions

---

### Scenario 2: School Computer Lab

**Setup 20 computers:**

Create a `deploy.bat`:
```batch
@echo off
SET COMPUTER_ID=%COMPUTERNAME%
familydesk.exe --device-id "%COMPUTER_ID%"
```

Run on each computer. They'll auto-register as:
- `LAB-PC-01`
- `LAB-PC-02`
- etc.

**Create IT admin account:**
```bash
curl -X POST "http://nas.haydenstudio.hk:21114/api/create-account" \
  -H "Content-Type: application/json" \
  -d '{
    "admin_password": "admin123",
    "new_username": "it_admin",
    "new_password": "SecureIT2025!",
    "role": "admin",
    "can_modify_settings": true,
    "device_ids": ["*"]
  }'
```

Now IT admin can manage all computers with one account!

---

## 🔍 Testing

### Test API Server

```bash
# Health check
curl http://nas.haydenstudio.hk:21114/health
# Should return: OK

# Test login
curl -X POST "http://nas.haydenstudio.hk:21114/api/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john",
    "password": "mypassword",
    "device_id": "GRANDMA_PC"
  }'
```

### Test Permission Change

1. Open FamilyDesk UI
2. Toggle any permission
3. Enter credentials
4. Check logs:
```bash
tail -f ~/.config/RustDesk/RustDesk.log
```

You should see:
```
Permission 'mouse' set to true by user 'john'
```

---

## 🎯 Current Status

**Your files are now in:**
```
/Users/hayden/Downloads/rustdesk2-master/
├── src/
│   ├── api_server/         ✓ Created
│   │   └── mod.rs
│   ├── api_client.rs       ✓ Created
│   └── simple_permissions.rs ✓ Created
├── tauri-src/              ✓ Created
│   ├── main.rs
│   ├── index.html
│   └── tauri.conf.json
├── manage_accounts.sh      ✓ Created
├── FAMILYDESK_README.md    ✓ Created
└── Cargo.toml              ✓ Updated
```

**Features added to Cargo.toml:**
- `family_desk` ✓
- `api_server` ✓
- `simple_permissions` ✓

**Dependencies added:**
- `actix-web` ✓
- `actix-rt` ✓
- `jsonwebtoken` ✓
- `argon2` ✓
- `reqwest` ✓

---

## 🔧 Next Steps

### 1. Change Admin Password

```bash
# TODO: Create a change_admin_password.sh script
# For now, modify src/api_server/mod.rs line 33
```

### 2. Enable HTTPS (Production)

Edit `src/api_server/mod.rs`:
```rust
HttpServer::new(|| { /* ... */ })
    .bind_openssl("0.0.0.0:21114", builder)?  // Add SSL
    .run()
    .await
```

### 3. Add Database (Optional)

Replace `lazy_static! ACCOUNTS` with PostgreSQL/SQLite:
```rust
use sqlx::PgPool;

let pool = PgPool::connect("postgres://...").await?;
```

### 4. Build Windows Installer

```bash
cd tauri-src
cargo tauri build --target x86_64-pc-windows-msvc
```

Output: `tauri-src/target/release/bundle/msi/FamilyDesk_1.0.0_x64_en-US.msi`

---

## ❓ Troubleshooting

**Problem:** Can't connect to API server
```bash
# Check if port is open
nc -zv nas.haydenstudio.hk 21114

# Check firewall
sudo ufw allow 21114
```

**Problem:** Authentication fails
```bash
# List all accounts
./manage_accounts.sh
# Choose option 2
```

**Problem:** Permissions not saving
```bash
# Check logs
cat ~/.config/RustDesk/RustDesk.log | grep Permission
```

---

## 📞 Support

Need help?
1. Check `FAMILYDESK_README.md` for detailed docs
2. Review API server logs
3. Test with `manage_accounts.sh`

**Default Credentials:**
- Username: `admin`
- Password: `admin123`

**API Endpoints:**
- Health: http://nas.haydenstudio.hk:21114/health
- Login: http://nas.haydenstudio.hk:21114/api/login
- Create Account: http://nas.haydenstudio.hk:21114/api/create-account

---

**You're all set! 🎉**

Run the API server, create accounts, and start helping your family or managing school computers!

<p align="center">
  <img src="res/logo-header.svg" alt="FamilyDesk - Remote desktop for family support"><br>
  <h1 align="center">FamilyDesk</h1>
  <p align="center">RustDesk Fork for Family & Educational Support</p>
</p>

<p align="center">
  <a href="#-key-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#️-configuration">Configuration</a> •
  <a href="#-testing">Testing</a>
</p>

---

## 📖 About

**FamilyDesk** is a specialized fork of RustDesk designed for:

- 👵 **Elderly Family Support** - Help elderly family members with simplified, accessible remote control
- 🏫 **School Computer Management** - Manage school computer labs with granular permission controls
- 🔒 **Transparent Access** - Consent-based remote access with audit logging and permission toggles

### Why FamilyDesk?

Traditional remote desktop tools can be overwhelming for non-technical users. FamilyDesk provides:

- Pre-configured servers (zero setup for end users)
- Simple permission toggles instead of complex settings
- Centralized account management
- Automatic server failover for reliability
- Clear audit trails for transparency

---

## 🎯 Key Features

| Feature | Description |
|---------|-------------|
| **Pre-configured Server** | Relay server `nas.haydenstudio.hk` works out of the box |
| **API Authentication** | Centralized account management with JWT tokens |
| **Permission Controls** | Toggle mouse, keyboard, clipboard, file transfer, audio |
| **Master Password** | Admin-only access to settings modification |
| **Server Failover** | Automatic retry across 1 primary + 5 standby servers |
| **Simplified UI** | Large, accessible interface designed for elderly users |
| **Role-Based Access** | Admin, family, and student roles with different permissions |
| **Audit Logging** | Track all permission changes and blocked access attempts |
| **Connection Pooling** | 5x performance improvement for API requests |
| **Health Checking** | Automatic server health monitoring every 30 seconds |

---

## 🚀 Quick Start

### Prerequisites

- Rust toolchain (1.70+)
- vcpkg (for dependencies)
- C++ build tools

### 1️⃣ Build FamilyDesk

```bash
cd /Users/hayden/Downloads/rustdesk2-master

# Build with FamilyDesk features
cargo build --features family_desk

# Or build optimized release version
cargo build --release --features family_desk
```

### 2️⃣ Start the API Server

```bash
# Start authentication server (required)
RUST_LOG=info cargo run --features family_desk --bin api_server
```

**Default Admin Account:**
- Username: `admin`
- Password: `admin123` ⚠️ **Change this immediately!**

API server will listen on `http://0.0.0.0:21114`

### 3️⃣ Run FamilyDesk Client

```bash
# Start FamilyDesk
RUST_LOG=info cargo run --features family_desk
```

### 4️⃣ (Optional) Start Tauri UI

```bash
# Simplified UI for elderly users
cd tauri-src
cargo tauri dev
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[FAMILYDESK_README.md](FAMILYDESK_README.md)** | Complete documentation with architecture details |
| **[QUICKSTART.md](QUICKSTART.md)** | 5-minute setup guide for new users |
| **[IMPLEMENTATION_COMPLETE.md](IMPLEMENTATION_COMPLETE.md)** | Full implementation summary with code statistics |
| **[CODE_REVIEW_AND_IMPROVEMENTS.md](CODE_REVIEW_AND_IMPROVEMENTS.md)** | Security review and improvement recommendations |
| **[CRITICAL_FIXES_APPLIED.md](CRITICAL_FIXES_APPLIED.md)** | List of all fixes and code changes |
| **[CONNECTION_POOLING_FIX.md](CONNECTION_POOLING_FIX.md)** | Performance optimization details |

---

## 🔒 Security Features

✅ **Argon2 Password Hashing** - Industry-standard password security
✅ **JWT Token Authentication** - Secure session management
✅ **Device-Level Restrictions** - Limit accounts to specific devices
✅ **Server-Side Enforcement** - Cannot be bypassed by modified clients
✅ **Audit Logging** - Track all actions with 🚫 emoji for blocked attempts
✅ **Role-Based Access Control** - Admin, family, and student roles

⚠️ **Before Production Deployment:**

1. 🔑 Change default admin password
2. 🔐 Configure HTTPS/TLS for API server
3. 💾 Set up persistent database (currently in-memory)
4. 🌐 Configure standby servers (5 slots available)
5. 🛡️ Review [CODE_REVIEW_AND_IMPROVEMENTS.md](CODE_REVIEW_AND_IMPROVEMENTS.md)

---

## ⚙️ Configuration

### Configure Standby Servers

Edit `src/api_server_config.rs` lines 54-83:

```rust
ServerConfig {
    url: "http://standby1.example.com:21114".to_string(),
    priority: 1,
    enabled: true,  // Enable standby server!
},
```

### Manage Permissions via API

**Set Permission:**
```bash
curl -X POST http://localhost:21114/api/set-permission \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123",
    "device_id": "123456789",
    "permission_name": "allow_mouse",
    "value": false
  }'
```

**Create User Account:**
```bash
curl -X POST http://localhost:21114/api/create-account \
  -H "Content-Type: application/json" \
  -d '{
    "username": "grandma",
    "password": "secure-password",
    "role": "family",
    "can_modify_settings": false,
    "device_ids": ["123456789"]
  }'
```

**Or use the interactive script:**
```bash
./manage_accounts.sh
```

---

## 🛠️ Feature Flags

| Flag | Description |
|------|-------------|
| `family_desk` | Enable all FamilyDesk features (meta-flag) |
| `api_server` | API server for centralized authentication |
| `simple_permissions` | Permission enforcement system |

**Build with specific features:**
```bash
cargo build --features "family_desk,api_server,simple_permissions"
```

---

## 🧪 Testing

### Test Permission Blocking

**1. Disable Mouse Control:**
```bash
# Via API
curl -X POST http://localhost:21114/api/set-permission \
  -d '{"username":"admin","password":"admin123","device_id":"123","permission_name":"allow_mouse","value":false}'

# Try moving mouse from remote → Should be blocked ✅
# Check logs: "🚫 Mouse blocked by FamilyDesk permissions"
```

**2. Test Keyboard:**
```bash
# Disable keyboard
curl -X POST http://localhost:21114/api/set-permission \
  -d '{"username":"admin","password":"admin123","device_id":"123","permission_name":"allow_keyboard","value":false}'

# Try typing from remote → Should be blocked ✅
```

**3. Test Re-enabling:**
```bash
# Enable mouse again
curl -X POST http://localhost:21114/api/set-permission \
  -d '{"username":"admin","password":"admin123","device_id":"123","permission_name":"allow_mouse","value":true}'

# Mouse should work immediately (no reconnection needed) ✅
```

### Test Server Failover

```bash
# Stop primary API server (Ctrl+C)
# Client should automatically try standby servers
# Check logs for: "Attempt 2/3 using server: http://standby1.example.com:21114"
```

### Run Unit Tests

```bash
cargo test --features family_desk
```

**See [IMPLEMENTATION_COMPLETE.md](IMPLEMENTATION_COMPLETE.md) for comprehensive testing guide.**

---

## 📊 Project Status

| Component | Status |
|-----------|--------|
| API Server | ✅ Complete |
| API Client | ✅ Complete |
| Permission System | ✅ Complete |
| Permission Enforcement | ✅ Integrated |
| Server Failover | ✅ Complete |
| Connection Pooling | ✅ Complete |
| Tauri UI | ✅ Complete |
| Documentation | ✅ Complete |
| Production Security | ⚠️ Requires hardening |
| Standby Servers | ⚠️ Needs configuration |

**Code Statistics:**
- Files Created: 17
- Files Modified: 4
- Lines Added: ~2,500+
- Documentation: ~2,000 lines

---

## 🤝 Contributing

This is a specialized fork for family/educational use. For the main RustDesk project, see [rustdesk/rustdesk](https://github.com/rustdesk/rustdesk).

---

## 📄 License

This project is a fork of [RustDesk](https://github.com/rustdesk/rustdesk) and maintains the same license.

---

## ⚖️ Disclaimer

> [!CAUTION]
> **Ethical Use Only**
>
> This software is designed for **legitimate family support and educational purposes** with explicit consent.
>
> **Prohibited uses:**
> - Unauthorized access to computers
> - Surveillance without consent
> - Privacy invasion
> - Any illegal activities
>
> The developers are not responsible for misuse of this software.

---

## 🔗 Original RustDesk Project

This is a fork of **RustDesk** - an open-source remote desktop solution written in Rust.

![image](https://user-images.githubusercontent.com/71636191/171661982-430285f0-2e12-4b1d-9957-4a58e375304d.png)

RustDesk welcomes contribution from everyone. See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for help getting started.

[**FAQ**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

[**BINARY DOWNLOAD**](https://github.com/rustdesk/rustdesk/releases)

[**NIGHTLY BUILD**](https://github.com/rustdesk/rustdesk/releases/tag/nightly)

[<img src="https://f-droid.org/badge/get-it-on.png"
    alt="Get it on F-Droid"
    height="80">](https://f-droid.org/en/packages/com.carriez.flutter_hbb)
[<img src="https://flathub.org/api/badge?svg&locale=en"
    alt="Get it on Flathub"
    height="80">](https://flathub.org/apps/com.rustdesk.RustDesk)

## Dependencies

Desktop versions use Flutter or Sciter (deprecated) for GUI, this tutorial is for Sciter only, since it is easier and more friendly to start. Check out our [CI](https://github.com/rustdesk/rustdesk/blob/master/.github/workflows/flutter-build.yml) for building Flutter version.

Please download Sciter dynamic library yourself.

[Windows](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.win/x64/sciter.dll) |
[Linux](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so) |
[macOS](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.osx/libsciter.dylib)

## Raw Steps to build

- Prepare your Rust development env and C++ build env

- Install [vcpkg](https://github.com/microsoft/vcpkg), and set `VCPKG_ROOT` env variable correctly

  - Windows: vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static
  - Linux/macOS: vcpkg install libvpx libyuv opus aom

- run `cargo run`

## [Build](https://rustdesk.com/docs/en/dev/build/)

## How to Build on Linux

### Ubuntu 18 (Debian 10)

```sh
sudo apt install -y zip g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev \
        libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake make \
        libclang-dev ninja-build libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libpam0g-dev
```

### openSUSE Tumbleweed

```sh
sudo zypper install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libXfixes-devel cmake alsa-lib-devel gstreamer-devel gstreamer-plugins-base-devel xdotool-devel pam-devel
```

### Fedora 28 (CentOS 8)

```sh
sudo yum -y install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libxdo-devel libXfixes-devel pulseaudio-libs-devel cmake alsa-lib-devel gstreamer1-devel gstreamer1-plugins-base-devel pam-devel
```

### Arch (Manjaro)

```sh
sudo pacman -Syu --needed unzip git cmake gcc curl wget yasm nasm zip make pkg-config clang gtk3 xdotool libxcb libxfixes alsa-lib pipewire
```

### Install vcpkg

```sh
git clone https://github.com/microsoft/vcpkg
cd vcpkg
git checkout 2023.04.15
cd ..
vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg
vcpkg/vcpkg install libvpx libyuv opus aom
```

### Fix libvpx (For Fedora)

```sh
cd vcpkg/buildtrees/libvpx/src
cd *
./configure
sed -i 's/CFLAGS+=-I/CFLAGS+=-fPIC -I/g' Makefile
sed -i 's/CXXFLAGS+=-I/CXXFLAGS+=-fPIC -I/g' Makefile
make
cp libvpx.a $HOME/vcpkg/installed/x64-linux/lib/
cd
```

### Build

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone --recurse-submodules https://github.com/rustdesk/rustdesk
cd rustdesk
mkdir -p target/debug
wget https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so
mv libsciter-gtk.so target/debug
VCPKG_ROOT=$HOME/vcpkg cargo run
```

## How to build with Docker

Begin by cloning the repository and building the Docker container:

```sh
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
git submodule update --init --recursive
docker build -t "rustdesk-builder" .
```

Then, each time you need to build the application, run the following command:

```sh
docker run --rm -it -v $PWD:/home/user/rustdesk -v rustdesk-git-cache:/home/user/.cargo/git -v rustdesk-registry-cache:/home/user/.cargo/registry -e PUID="$(id -u)" -e PGID="$(id -g)" rustdesk-builder
```

Note that the first build may take longer before dependencies are cached, subsequent builds will be faster. Additionally, if you need to specify different arguments to the build command, you may do so at the end of the command in the `<OPTIONAL-ARGS>` position. For instance, if you wanted to build an optimized release version, you would run the command above followed by `--release`. The resulting executable will be available in the target folder on your system, and can be run with:

```sh
target/debug/rustdesk
```

Or, if you're running a release executable:

```sh
target/release/rustdesk
```

Please ensure that you run these commands from the root of the RustDesk repository, or the application may not find the required resources. Also note that other cargo subcommands such as `install` or `run` are not currently supported via this method as they would install or run the program inside the container instead of the host.

## File Structure

- **[libs/hbb_common](https://github.com/rustdesk/rustdesk/tree/master/libs/hbb_common)**: video codec, config, tcp/udp wrapper, protobuf, fs functions for file transfer, and some other utility functions
- **[libs/scrap](https://github.com/rustdesk/rustdesk/tree/master/libs/scrap)**: screen capture
- **[libs/enigo](https://github.com/rustdesk/rustdesk/tree/master/libs/enigo)**: platform specific keyboard/mouse control
- **[libs/clipboard](https://github.com/rustdesk/rustdesk/tree/master/libs/clipboard)**: file copy and paste implementation for Windows, Linux, macOS.
- **[src/ui](https://github.com/rustdesk/rustdesk/tree/master/src/ui)**: obsolete Sciter UI (deprecated)
- **[src/server](https://github.com/rustdesk/rustdesk/tree/master/src/server)**: audio/clipboard/input/video services, and network connections
- **[src/client.rs](https://github.com/rustdesk/rustdesk/tree/master/src/client.rs)**: start a peer connection
- **[src/rendezvous_mediator.rs](https://github.com/rustdesk/rustdesk/tree/master/src/rendezvous_mediator.rs)**: Communicate with [rustdesk-server](https://github.com/rustdesk/rustdesk-server), wait for remote direct (TCP hole punching) or relayed connection
- **[src/platform](https://github.com/rustdesk/rustdesk/tree/master/src/platform)**: platform specific code
- **[flutter](https://github.com/rustdesk/rustdesk/tree/master/flutter)**: Flutter code for desktop and mobile
- **[flutter/web/js](https://github.com/rustdesk/rustdesk/tree/master/flutter/web/v1/js)**: JavaScript for Flutter web client

## Screenshots

![Connection Manager](https://github.com/rustdesk/rustdesk/assets/28412477/db82d4e7-c4bc-4823-8e6f-6af7eadf7651)

![Connected to a Windows PC](https://github.com/rustdesk/rustdesk/assets/28412477/9baa91e9-3362-4d06-aa1a-7518edcbd7ea)

![File Transfer](https://github.com/rustdesk/rustdesk/assets/28412477/39511ad3-aa9a-4f8c-8947-1cce286a46ad)

![TCP Tunneling](https://github.com/rustdesk/rustdesk/assets/28412477/78e8708f-e87e-4570-8373-1360033ea6c5)


# FamilyDesk 多平台支持

FamilyDesk 支持 5 个主要平台的构建和部署。

---

## 📊 支持的平台

| 平台 | 架构 | 状态 | GitHub Workflow |
|------|------|------|----------------|
| 🍎 **macOS** | Intel (x64) + Apple Silicon (ARM64) | ✅ 完全支持 | `build-macos.yml` |
| 🪟 **Windows** | x64 | ✅ 完全支持 | `build-windows.yml` |
| 🐧 **Linux** | x64 | ✅ 完全支持 | `build-linux.yml` |
| 🤖 **Android** | ARM64, ARMv7, x64 | ✅ 完全支持 | `build-android.yml` |
| 📱 **iOS** | ARM64 | ⚠️ 未签名 | `build-ios.yml` |

---

## 🚀 快速构建

### 选项 1: 构建单个平台

访问: `https://github.com/[用户名]/haydendesk/actions`

选择对应平台的 workflow:
- **Build FamilyDesk - macOS** → macOS 可执行文件
- **Build FamilyDesk - Windows** → Windows .exe
- **Build FamilyDesk - Linux** → Linux binary
- **Build FamilyDesk - Android** → APK + AAB
- **Build FamilyDesk - iOS** → IPA (未签名)

### 选项 2: 一次构建所有平台

选择 **Build FamilyDesk - All Platforms** workflow，然后:
- 输入 `all` → 构建所有 5 个平台
- 输入 `macos,windows,linux` → 只构建桌面版
- 输入 `android,ios` → 只构建移动版

---

## 📦 各平台产物

### macOS
```
familydesk-macos-1.4.2/
└── rustdesk (可执行文件，~50MB)
```

**如何使用:**
```bash
chmod +x rustdesk
./rustdesk
```

**安装位置建议:** `/Applications/FamilyDesk.app`

---

### Windows
```
familydesk-windows-1.4.2/
└── rustdesk.exe (~30MB)
```

**如何使用:**
双击 `rustdesk.exe` 运行

**安装位置建议:** `C:\Program Files\FamilyDesk\`

---

### Linux
```
familydesk-linux-1.4.2/
└── rustdesk (ELF binary, ~40MB)
```

**如何使用:**
```bash
chmod +x rustdesk
./rustdesk
```

**安装位置建议:** `/usr/local/bin/familydesk`

**系统要求:**
- GTK3
- X11
- ALSA/PulseAudio

---

### Android
```
familydesk-android-1.4.2/
└── app-release.apk (~20MB)

familydesk-android-bundle-1.4.2/
└── app-release.aab (Google Play 上传用)
```

**如何安装 APK:**
```bash
# 使用 adb
adb install app-release.apk

# 或直接传输到设备并安装
```

**支持的架构:**
- ARM64 (主要)
- ARMv7
- x86_64 (模拟器)

**最低 Android 版本:** Android 6.0 (API 23)

---

### iOS
```
familydesk-ios-1.4.2-unsigned/
└── FamilyDesk.ipa (~30MB, 未签名)

familydesk-ios-app-1.4.2/
└── Runner.app/ (用于模拟器)
```

**⚠️ 重要提示:**
- IPA 文件**未签名**，不能直接安装到真机
- 需要 Apple Developer 账户和证书才能签名
- `Runner.app` 可用于 Xcode 模拟器测试

**如何在模拟器中测试:**
```bash
# 启动模拟器
open -a Simulator

# 安装 app
xcrun simctl install booted Runner.app
```

**如何签名 (需要开发者账户):**
```bash
codesign --force --sign "Your Developer ID" FamilyDesk.ipa
```

---

## 🛠️ 本地构建

如果不想使用 GitHub Actions，也可以本地构建:

### macOS
```bash
./clean-and-build.sh
# 或
cargo build --features family_desk --release
```

### Windows
```powershell
cargo build --features family_desk --release
```

### Linux
```bash
cargo build --features family_desk --release
```

### Android
```bash
cd flutter
flutter build apk --release
```

### iOS
```bash
cd flutter
flutter build ios --release --no-codesign
```

---

## 📋 功能对比

| 功能 | macOS | Windows | Linux | Android | iOS |
|------|-------|---------|-------|---------|-----|
| **远程桌面控制** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **文件传输** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **剪贴板同步** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **API 认证** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **权限管理** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **服务器故障转移** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **系统托盘** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **后台服务** | ✅ | ✅ | ✅ | 部分 | 部分 |

---

## 🔧 系统要求

### macOS
- **最低版本:** macOS 10.13 (High Sierra)
- **推荐版本:** macOS 12.3+
- **内存:** 2GB+
- **存储:** 100MB

### Windows
- **最低版本:** Windows 7
- **推荐版本:** Windows 10/11
- **架构:** x64
- **内存:** 2GB+
- **存储:** 50MB
- **依赖:** Visual C++ Redistributable

### Linux
- **发行版:** Ubuntu 18.04+, Debian 10+, Fedora 30+
- **桌面环境:** GNOME, KDE, XFCE
- **依赖:**
  - GTK3
  - X11 或 Wayland
  - ALSA/PulseAudio
- **内存:** 2GB+
- **存储:** 80MB

### Android
- **最低版本:** Android 6.0 (API 23)
- **推荐版本:** Android 8.0+
- **架构:** ARM64 (主要), ARMv7, x86_64
- **内存:** 2GB+
- **存储:** 50MB
- **权限:**
  - 网络访问
  - 存储读写（文件传输）
  - 麦克风（可选）

### iOS
- **最低版本:** iOS 12.0
- **推荐版本:** iOS 14.0+
- **架构:** ARM64
- **内存:** 2GB+
- **存储:** 80MB

---

## 🌐 服务器配置

所有平台都使用相同的服务器配置:

```
主服务器: nas.haydenstudio.hk
API 服务器: http://nas.haydenstudio.hk:21114
公钥: iQX+6aUbS40CL9jJElm4jSMs8aKzePqMsFH1HACDI5E=
```

备用服务器需要在 `src/api_server_config.rs` 中配置。

---

## 📱 移动平台特别说明

### Android 特性
- ✅ 支持 ARMv7 和 ARM64
- ✅ 可通过 Google Play 或 APK 分发
- ✅ 支持后台服务 (有限制)
- ⚠️ 需要电池优化白名单

### iOS 特性
- ⚠️ 构建产物未签名
- ⚠️ 后台运行受限 (iOS 限制)
- ⚠️ 需要 App Store 或 TestFlight 分发
- ✅ 支持 VoIP 推送保持连接

---

## 🔍 常见问题

### Q: 为什么 iOS 版本未签名？
A: Apple 签名需要付费开发者账户和证书。GitHub Actions 无法自动签名。你需要在本地使用 Xcode 或 `codesign` 命令签名。

### Q: Android APK 和 AAB 有什么区别？
A:
- **APK** - 直接安装到设备，可通过网站分发
- **AAB** - 上传到 Google Play，由 Google 优化后分发

### Q: Linux 版本需要哪些依赖？
A: GTK3, X11, ALSA/PulseAudio。大多数桌面发行版已预装。

### Q: macOS 版本支持 M1/M2/M3 芯片吗？
A: 是的，GitHub Actions 构建 x64 版本，但可通过 Rosetta 2 运行。也可以本地构建 ARM64 原生版本。

### Q: 如何创建 macOS .app 包？
A: 需要手动创建 `.app` 结构并签名。详见 macOS 打包文档。

---

## 📚 相关文档

- [.github/workflows/README.md](.github/workflows/README.md) - Workflows 详细说明
- [WORKFLOW_QUICK_START.md](WORKFLOW_QUICK_START.md) - 快速开始指南
- [BUILD_TROUBLESHOOTING.md](BUILD_TROUBLESHOOTING.md) - 构建问题排查

---

**状态:** ✅ 5 个平台完全支持
**最后更新:** 2025-02-10
**版本:** 1.4.2

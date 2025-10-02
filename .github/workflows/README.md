# GitHub Actions Workflows

## 📋 可用的 Workflows

FamilyDesk 支持多平台构建，每个平台都有独立的 workflow：

### 🖥️ 桌面平台

#### 1. `build-macos.yml` - macOS 构建
- **支持:** macOS (Intel & Apple Silicon)
- **产物:** `familydesk-macos-1.4.2`
- **Runner:** macos-13

#### 2. `build-windows.yml` - Windows 构建
- **支持:** Windows x64
- **产物:** `familydesk-windows-1.4.2` (.exe)
- **Runner:** windows-2022

#### 3. `build-linux.yml` - Linux 构建
- **支持:** Ubuntu/Debian x64
- **产物:** `familydesk-linux-1.4.2`
- **Runner:** ubuntu-20.04

### 📱 移动平台

#### 4. `build-android.yml` - Android 构建
- **支持:** Android (ARM64, ARMv7, x64)
- **产物:**
  - APK: `familydesk-android-1.4.2`
  - AAB: `familydesk-android-bundle-1.4.2`
- **Runner:** ubuntu-20.04

#### 5. `build-ios.yml` - iOS 构建
- **支持:** iOS (ARM64)
- **产物:** `familydesk-ios-1.4.2-unsigned` (.ipa)
- **注意:** 未签名版本，需要开发者证书才能安装
- **Runner:** macos-13

### 🚀 统一构建

#### 6. `build-all-platforms.yml` - 所有平台
- **用途:** 一次性构建所有平台或选择性构建
- **输入参数:** 选择要构建的平台
  - `all` - 构建所有平台
  - `macos,windows,linux` - 只构建桌面平台
  - `android,ios` - 只构建移动平台
- **调用其他 workflows**

### 🧹 工具

#### 7. `clear-cache.yml` - 清除缓存
- **用途:** 清除 GitHub Actions 缓存

---

## 🚀 快速开始

### 方法 1: 构建单个平台

**步骤:**
1. 访问 `https://github.com/[用户名]/haydendesk/actions`
2. 选择要构建的平台:
   - `Build FamilyDesk - macOS`
   - `Build FamilyDesk - Windows`
   - `Build FamilyDesk - Linux`
   - `Build FamilyDesk - Android`
   - `Build FamilyDesk - iOS`
3. 点击 "Run workflow"
4. 选择分支
5. 点击绿色 "Run workflow" 按钮

### 方法 2: 构建所有平台

**步骤:**
1. 访问 Actions 页面
2. 选择 `Build FamilyDesk - All Platforms`
3. 点击 "Run workflow"
4. 在 "platforms" 输入框中:
   - 输入 `all` 构建所有平台
   - 输入 `macos,windows,linux` 只构建桌面版
   - 输入 `android,ios` 只构建移动版
5. 点击绿色 "Run workflow" 按钮

### 方法 3: GitHub CLI

```bash
# 构建 macOS
gh workflow run build-macos.yml

# 构建 Windows
gh workflow run build-windows.yml

# 构建 Linux
gh workflow run build-linux.yml

# 构建 Android
gh workflow run build-android.yml

# 构建 iOS
gh workflow run build-ios.yml

# 构建所有平台
gh workflow run build-all-platforms.yml -f platforms=all

# 只构建桌面平台
gh workflow run build-all-platforms.yml -f platforms=macos,windows,linux
```

---

## 📊 平台对比

| 平台 | 构建时间 | 产物大小 | 依赖复杂度 | 状态 |
|------|---------|---------|-----------|------|
| **macOS** | ~15-30分钟 | ~50MB | 中等 | ✅ 可用 |
| **Windows** | ~20-35分钟 | ~30MB | 中等 | ✅ 可用 |
| **Linux** | ~15-25分钟 | ~40MB | 低 | ✅ 可用 |
| **Android** | ~25-40分钟 | ~20MB (APK) | 高 | ✅ 可用 |
| **iOS** | ~20-35分钟 | ~30MB | 高 | ⚠️ 未签名 |

---

## 🔍 各平台构建详情

### macOS 构建

**依赖:**
- Rust 1.75
- Homebrew (llvm, nasm, pkg-config, glib, gtk+3, cairo)
- vcpkg (opus, vpx, yuv, aom)

**Features:**
```bash
cargo build --features family_desk --release
```

**验证:**
- ✅ PKG_CONFIG_PATH 设置
- ✅ glib-2.0 可用
- ✅ vcpkg manifest 模式

---

### Windows 构建

**依赖:**
- Rust 1.75 (MSVC target)
- LLVM/Clang
- vcpkg (x64-windows-static triplet)

**Features:**
```bash
cargo build --features family_desk --release
```

**产物:**
- `rustdesk.exe`

---

### Linux 构建

**依赖:**
- Rust 1.75
- GTK3, X11, ALSA, PulseAudio
- vcpkg (x64-linux triplet)

**Features:**
```bash
cargo build --features family_desk --release
```

**产物:**
- `rustdesk` (ELF binary)
- AppDir 结构（可选）

---

### Android 构建

**依赖:**
- Flutter 3.13.9
- Android NDK r26b
- Java 11
- Rust (Android targets)

**构建:**
```bash
flutter build apk --release --target-platform android-arm64
flutter build appbundle --release
```

**产物:**
- `app-release.apk` (直接安装)
- `app-release.aab` (Google Play)

---

### iOS 构建

**依赖:**
- Flutter 3.13.9
- Xcode 15.0
- CocoaPods
- Rust (iOS targets)

**构建:**
```bash
flutter build ios --release --no-codesign
```

**产物:**
- `Runner.app`
- `FamilyDesk.ipa` (未签名)

**注意:**
- ⚠️ 需要 Apple Developer 证书才能安装到真机
- 可用于模拟器测试
- 正式发布需要签名

---

## 🔧 故障排除

### macOS

**问题:** glib-2.0 找不到
**解决:** 检查 "Verify pkg-config setup" 步骤

**问题:** vcpkg manifest 错误
**解决:** 确保使用正确的 baseline (120deac3...)

### Windows

**问题:** MSVC 链接错误
**解决:** 确保安装了 Visual Studio Build Tools

**问题:** vcpkg 安装失败
**解决:** 检查网络连接，重试

### Linux

**问题:** 缺少系统库
**解决:** 检查 "Install system dependencies" 步骤

### Android

**问题:** NDK 路径错误
**解决:** 使用 setup-ndk action 自动配置

**问题:** Flutter 构建失败
**解决:** 检查 pubspec.yaml 依赖

### iOS

**问题:** Pod install 失败
**解决:** 检查 CocoaPods 版本，清除缓存

**问题:** Xcode 签名错误
**解决:** 使用 --no-codesign 跳过签名

---

## 📝 修改 Workflows

### 添加新的构建步骤

编辑对应平台的 `.yml` 文件:

```yaml
- name: 你的新步骤
  run: |
    # 你的命令
```

### 修改 Feature Flags

在 "Build FamilyDesk" 步骤中:

```yaml
- name: Build FamilyDesk
  run: |
    cargo build --features family_desk,your_feature --release
```

### 添加环境变量

在文件顶部 `env:` 部分:

```yaml
env:
  YOUR_VAR: "value"
```

---

## 📚 相关文档

- [WORKFLOW_QUICK_START.md](../../WORKFLOW_QUICK_START.md) - 快速开始
- [GITHUB_ACTIONS_USAGE.md](../../GITHUB_ACTIONS_USAGE.md) - 详细说明
- [BUILD_TROUBLESHOOTING.md](../../BUILD_TROUBLESHOOTING.md) - 问题排查

---

## 📦 发布流程推荐

1. **开发阶段:** 本地构建测试
2. **测试阶段:** 单平台 workflow 测试
3. **预发布:** 使用 `build-all-platforms.yml` 构建所有平台
4. **发布:** 创建 GitHub Release 并附加产物

---

**状态:** ✅ 支持 5 个平台
**最后更新:** 2025-02-10
**推荐:** 使用平台专用 workflow 或 build-all-platforms.yml

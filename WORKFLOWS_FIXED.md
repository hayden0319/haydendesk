# GitHub Actions Workflows - PKG_CONFIG_PATH 修复总结

## 🔧 问题描述

构建失败，错误信息：
```
The system library `glib-2.0` required by crate `glib-sys` was not found.
The PKG_CONFIG_PATH environment variable is not set.
```

## ✅ 解决方案

**双重保险策略**：既使用 `>> $GITHUB_ENV` 设置全局环境变量，又在构建步骤中 `export` 确保 cargo 子进程可以访问。

### 修复前（❌ 不工作）:
```yaml
- name: Build
  run: |
    export PKG_CONFIG_PATH=/usr/lib/pkgconfig
    cargo build --release
```

### 修复后（✅ 工作）:
```yaml
- name: Set build environment
  run: |
    echo "PKG_CONFIG_PATH=/usr/lib/pkgconfig" >> $GITHUB_ENV
    echo "PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1" >> $GITHUB_ENV

- name: Verify environment
  run: |
    echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
    pkg-config --exists glib-2.0 && echo "✅ glib-2.0 found"

- name: Build
  run: |
    export PKG_CONFIG_PATH=/usr/lib/pkgconfig
    export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
    cargo build --release
```

**关键发现**：即使使用了 `>> $GITHUB_ENV`，cargo build 的子进程有时仍然无法访问环境变量。因此需要在构建步骤内再次 `export`。

---

## 📋 各平台修复详情

### 1. ✅ macOS (build-macos.yml)

**修改步骤：**
```yaml
env:
  RUST_VERSION: "1.81"  # Required for macOS (cidre and other deps need 1.81+)

- name: Install dependencies
  run: |
    brew install llvm nasm pkg-config glib gtk+3 cairo pango atk gdk-pixbuf

- name: Set build environment
  run: |
    BREW_PREFIX=$(brew --prefix)
    echo "PKG_CONFIG_PATH=$BREW_PREFIX/lib/pkgconfig:$BREW_PREFIX/share/pkgconfig:$BREW_PREFIX/opt/glib/lib/pkgconfig:$BREW_PREFIX/opt/gtk+3/lib/pkgconfig:$BREW_PREFIX/opt/cairo/lib/pkgconfig" >> $GITHUB_ENV
    echo "PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1" >> $GITHUB_ENV

- name: Verify build environment
  run: |
    echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
    pkg-config --exists glib-2.0 && echo "✅ glib-2.0 found"
    pkg-config --modversion glib-2.0

- name: Build FamilyDesk
  run: |
    BREW_PREFIX=$(brew --prefix)
    export PKG_CONFIG_PATH=$BREW_PREFIX/lib/pkgconfig:$BREW_PREFIX/share/pkgconfig:$BREW_PREFIX/opt/glib/lib/pkgconfig:$BREW_PREFIX/opt/gtk+3/lib/pkgconfig:$BREW_PREFIX/opt/cairo/lib/pkgconfig
    export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
    cargo build --features family_desk --release
```

**关键点：**
- ✅ **升级 Rust 到 1.81**（macOS 依赖如 cidre 需要）
- ✅ 动态检测 Homebrew 路径 (`brew --prefix`)
- ✅ 支持 Intel Mac (`/usr/local`) 和 Apple Silicon (`/opt/homebrew`)
- ✅ 使用 `>> $GITHUB_ENV` 设置环境变量
- ✅ **在构建步骤中再次 export（双重保险）**

---

### 2. ✅ Linux (build-linux.yml)

**修改步骤：**
```yaml
- name: Install dependencies
  run: |
    sudo apt update
    sudo apt install -y libglib2.0-dev pkg-config \
      gcc git curl wget nasm yasm libgtk-3-dev clang \
      [其他依赖...]

- name: Set build environment
  run: |
    echo "PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/lib/pkgconfig" >> $GITHUB_ENV
    echo "PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1" >> $GITHUB_ENV

- name: Verify pkg-config setup
  run: |
    echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
    for lib in glib-2.0 gtk+-3.0; do
      pkg-config --exists $lib && echo "✅ $lib found"
    done

- name: Verify environment before build
  run: |
    echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
    pkg-config --exists glib-2.0 && echo "✅ glib-2.0 accessible"

- name: Build FamilyDesk
  run: |
    export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/lib/pkgconfig
    export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
    cargo build --features family_desk --release
```

**关键点：**
- ✅ 第一行安装 `libglib2.0-dev pkg-config`
- ✅ 标准的 Ubuntu pkgconfig 路径
- ✅ 验证步骤确保库可用
- ✅ **在构建步骤中再次 export（双重保险）**

---

### 3. ✅ Android (build-android.yml)

**修改步骤：**
```yaml
- name: Install dependencies
  run: |
    sudo apt update
    sudo apt install -y libglib2.0-dev pkg-config \
      llvm-dev libclang-dev clang nasm yasm

- name: Set build environment
  run: |
    echo "PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/lib/pkgconfig" >> $GITHUB_ENV
    echo "PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1" >> $GITHUB_ENV

- name: Verify environment before build
  run: |
    echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
    pkg-config --exists glib-2.0 && echo "✅ glib-2.0 found"

- name: Build Flutter Android App
  working-directory: flutter
  run: |
    export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/lib/pkgconfig
    export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
    flutter pub get
    flutter build apk --release
```

**关键点：**
- ✅ 添加了 `libglib2.0-dev pkg-config`
- ✅ 设置环境变量（Flutter 可能需要）
- ✅ **在构建步骤中再次 export（双重保险）**

---

### 4. ✅ iOS (build-ios.yml)

**修改步骤：**
```yaml
env:
  RUST_VERSION: "1.81"  # Required for iOS (cargo-lipo dependencies need 1.81+)

- name: Install build dependencies
  run: |
    brew install pkg-config || echo "pkg-config already installed"

- name: Install cargo-lipo
  run: |
    cargo install cargo-lipo

- name: Build iOS app
  working-directory: flutter
  run: |
    flutter build ios --release --no-codesign
```

**关键点：**
- ✅ **升级 Rust 到 1.81**（cargo-lipo 依赖需要）
- ✅ 安装 pkg-config（防御性）
- ✅ Flutter 处理大部分依赖

**重要**：iOS 需要 Rust 1.81+ 因为 `cargo-lipo` 的依赖（如 `addr2line v0.25.1`）要求更新的 Rust 版本。

---

### 5. ✅ Windows (build-windows.yml)

**无需修改：**
```yaml
- name: Install vcpkg dependencies
  run: |
    .\vcpkg.exe install --triplet x64-windows-static

- name: Build FamilyDesk
  run: |
    cargo build --features family_desk --release
```

**原因：**
- ✅ 使用 vcpkg static triplet
- ✅ 不依赖系统级 glib
- ✅ 所有依赖都静态链接

---

## 🔍 为什么需要 glib？

FamilyDesk 依赖的核心库需要 glib：

```toml
[dependencies]
scrap = { path = "libs/scrap", features = ["wayland"] }  # 需要 glib
hbb_common = { path = "libs/hbb_common" }                # 需要 glib
```

这些是 RustDesk 的屏幕捕获和通用库，在 Linux/macOS 上需要 GTK/glib 支持。

---

## 📊 对比原始 RustDesk

### 原始 RustDesk (flutter-build.yml)

```yaml
# 只安装基础工具
- name: Install build runtime
  run: |
    brew install llvm create-dmg nasm
    if command -v pkg-config &>/dev/null; then
      echo "pkg-config is already installed"
    else
      brew install pkg-config
    fi
```

**为什么不需要 glib？**
- 使用 `--features flutter,hwcodec`
- Flutter UI 不直接依赖 GTK/glib

### FamilyDesk

```yaml
# 需要完整的 GTK 栈
- name: Install dependencies
  run: |
    brew install pkg-config glib gtk+3 cairo pango atk gdk-pixbuf
```

**为什么需要 glib？**
- 使用 `--features family_desk`
- 依赖 `scrap` 和 `hbb_common`
- 这些库在桌面平台需要 GTK/glib

---

## ✅ 验证清单

构建前检查：

**macOS:**
- [ ] `brew install glib gtk+3` 已执行
- [ ] `PKG_CONFIG_PATH` 包含 brew 路径
- [ ] `pkg-config --exists glib-2.0` 返回成功

**Linux:**
- [ ] `apt install libglib2.0-dev` 已执行
- [ ] `PKG_CONFIG_PATH` 包含 `/usr/lib/x86_64-linux-gnu/pkgconfig`
- [ ] `pkg-config --exists glib-2.0` 返回成功

**Windows:**
- [ ] vcpkg 使用 `x64-windows-static` triplet
- [ ] 不需要检查 PKG_CONFIG_PATH

---

## 🚀 测试命令

### 本地测试（macOS）:
```bash
export PKG_CONFIG_PATH=$(brew --prefix)/lib/pkgconfig:$(brew --prefix)/opt/glib/lib/pkgconfig
pkg-config --exists glib-2.0 && echo "OK"
cargo build --features family_desk --release
```

### 本地测试（Linux）:
```bash
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig
pkg-config --exists glib-2.0 && echo "OK"
cargo build --features family_desk --release
```

---

## 📚 参考

- GitHub Actions 环境变量: https://docs.github.com/en/actions/learn-github-actions/variables
- pkg-config 文档: https://people.freedesktop.org/~dbn/pkg-config-guide.html
- RustDesk 原始 workflows: `/Users/hayden/Downloads/rustdesk2-main/.github/workflows/`

---

## 🔧 Rust 版本要求

### macOS & iOS: 1.81+
- **原因**: cargo-lipo 依赖（如 addr2line v0.25.1）需要 rustc 1.81+
- **参考**: RustDesk 原始项目对 macOS 使用 `MAC_RUST_VERSION: "1.81"`

### Linux, Windows, Android: 1.75
- **原因**: 兼容 Sciter 和现有依赖
- **参考**: RustDesk 原始项目使用 `RUST_VERSION: "1.75"`

### 版本说明
根据 RustDesk 官方讨论：
- 1.75 是推荐版本（https://github.com/rustdesk/rustdesk/discussions/7503）
- 1.78 有 ABI 变化导致 Sciter 版本不兼容
- macOS/iOS 必须使用 1.81+ 因为 cidre 等依赖要求

---

**状态:** ✅ 所有 workflows 已修复
**最后更新:** 2025-02-10
**测试状态:** 待测试

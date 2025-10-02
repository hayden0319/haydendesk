#!/bin/bash

# FamilyDesk 環境設置腳本
# 自動安裝所有必需的依賴

set -e

echo "======================================================================"
echo "           FamilyDesk 環境設置                                        "
echo "======================================================================"
echo ""

# 檢測操作系統
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macOS"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="Linux"
else
    echo "❌ 不支持的操作系統: $OSTYPE"
    exit 1
fi

echo "檢測到操作系統: $OS"
echo ""

# ============================================================
# 1. 檢查並安裝 Rust
# ============================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1️⃣  檢查 Rust 安裝狀態"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if command -v cargo &> /dev/null; then
    echo "✅ Rust 已安裝"
    echo "   版本: $(rustc --version)"
    echo "   Cargo: $(cargo --version)"
else
    echo "📦 Rust 未安裝，正在安裝..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # 載入 Rust 環境
    source $HOME/.cargo/env

    echo "✅ Rust 安裝完成"
    echo "   版本: $(rustc --version)"
fi

echo ""

# ============================================================
# 2. 檢查並安裝系統依賴
# ============================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2️⃣  檢查系統依賴"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [[ "$OS" == "macOS" ]]; then
    # 檢查 Homebrew
    if ! command -v brew &> /dev/null; then
        echo "📦 Homebrew 未安裝，正在安裝..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    else
        echo "✅ Homebrew 已安裝"
    fi

    # 安裝依賴
    echo "📦 安裝構建依賴..."
    brew install llvm nasm pkg-config glib gtk+3 cairo pango atk gdk-pixbuf

elif [[ "$OS" == "Linux" ]]; then
    # 檢測 Linux 發行版
    if [ -f /etc/debian_version ]; then
        echo "檢測到 Debian/Ubuntu"
        sudo apt update
        sudo apt install -y gcc git curl wget nasm yasm \
            libgtk-3-dev clang libxcb-randr0-dev libxdo-dev \
            libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev \
            libasound2-dev libpulse-dev cmake
    elif [ -f /etc/fedora-release ]; then
        echo "檢測到 Fedora"
        sudo dnf install -y gcc-c++ git curl wget nasm yasm gcc \
            gtk3-devel clang libxcb-devel libxdo-devel libXfixes-devel \
            pulseaudio-libs-devel cmake alsa-lib-devel
    else
        echo "⚠️  未知的 Linux 發行版，請手動安裝依賴"
    fi
fi

echo "✅ 系統依賴安裝完成"

# 設置 PKG_CONFIG_PATH (macOS)
if [[ "$OS" == "macOS" ]]; then
    if command -v brew &> /dev/null; then
        BREW_PREFIX=$(brew --prefix)
        export PKG_CONFIG_PATH="$BREW_PREFIX/lib/pkgconfig:$BREW_PREFIX/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
        echo "✅ PKG_CONFIG_PATH 已設置: $PKG_CONFIG_PATH"
    fi
fi

echo ""

# ============================================================
# 3. 設置 vcpkg
# ============================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3️⃣  設置 vcpkg"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

VCPKG_DIR="$HOME/vcpkg"
VCPKG_COMMIT="120deac3062162151622ca4860575a33844ba10b"

if [ -d "$VCPKG_DIR" ]; then
    echo "✅ vcpkg 目錄已存在: $VCPKG_DIR"
    cd "$VCPKG_DIR"

    # 檢查是否在正確的 commit
    CURRENT_COMMIT=$(git rev-parse HEAD)
    if [ "$CURRENT_COMMIT" != "$VCPKG_COMMIT" ]; then
        echo "📦 更新 vcpkg 到正確版本..."
        git fetch
        git checkout $VCPKG_COMMIT
        ./bootstrap-vcpkg.sh
    fi
else
    echo "📦 克隆 vcpkg..."
    git clone https://github.com/microsoft/vcpkg "$VCPKG_DIR"
    cd "$VCPKG_DIR"

    echo "📦 檢出正確版本..."
    git checkout $VCPKG_COMMIT

    echo "📦 初始化 vcpkg..."
    ./bootstrap-vcpkg.sh
fi

echo "✅ vcpkg 設置完成"
echo ""

# 設置環境變量
export VCPKG_ROOT="$VCPKG_DIR"

# ============================================================
# 4. 安裝 vcpkg 依賴
# ============================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4️⃣  安裝 vcpkg 依賴（從 vcpkg.json）"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 返回項目目錄
cd "$(dirname "$0")"

# 檢測架構
if [[ "$OS" == "macOS" ]]; then
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ]; then
        TRIPLET="arm64-osx"
        echo "檢測到 Apple Silicon (ARM64)"
    else
        TRIPLET="x64-osx"
        echo "檢測到 Intel Mac (x86_64)"
    fi
elif [[ "$OS" == "Linux" ]]; then
    TRIPLET="x64-linux"
fi

echo "使用 triplet: $TRIPLET"
echo ""
echo "📦 安裝依賴（這可能需要一些時間）..."

$VCPKG_ROOT/vcpkg install --triplet $TRIPLET

echo "✅ vcpkg 依賴安裝完成"
echo ""

# ============================================================
# 5. 保存環境變量
# ============================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "5️⃣  保存環境變量"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 檢測 shell
if [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
else
    SHELL_RC="$HOME/.profile"
fi

# 檢查是否已經添加
if grep -q "VCPKG_ROOT" "$SHELL_RC" 2>/dev/null; then
    echo "✅ VCPKG_ROOT 已在 $SHELL_RC 中配置"
else
    echo "📝 添加 VCPKG_ROOT 到 $SHELL_RC"
    echo "" >> "$SHELL_RC"
    echo "# vcpkg - added by FamilyDesk setup" >> "$SHELL_RC"
    echo "export VCPKG_ROOT=\"$VCPKG_DIR\"" >> "$SHELL_RC"
    echo "✅ 已添加到 $SHELL_RC"
fi

# 檢查 Rust 環境
if grep -q ".cargo/env" "$SHELL_RC" 2>/dev/null; then
    echo "✅ Rust 環境已配置"
else
    echo "📝 添加 Rust 環境到 $SHELL_RC"
    echo "source \$HOME/.cargo/env" >> "$SHELL_RC"
fi

# 檢查 PKG_CONFIG_PATH (macOS)
if [[ "$OS" == "macOS" ]]; then
    if ! grep -q "PKG_CONFIG_PATH" "$SHELL_RC" 2>/dev/null; then
        echo "📝 添加 PKG_CONFIG_PATH 到 $SHELL_RC"
        BREW_PREFIX=$(brew --prefix 2>/dev/null || echo "/opt/homebrew")
        echo "export PKG_CONFIG_PATH=\"$BREW_PREFIX/lib/pkgconfig:$BREW_PREFIX/share/pkgconfig\${PKG_CONFIG_PATH:+:\$PKG_CONFIG_PATH}\"" >> "$SHELL_RC"
        echo "✅ 已添加 PKG_CONFIG_PATH"
    else
        echo "✅ PKG_CONFIG_PATH 已配置"
    fi
fi

echo ""

# ============================================================
# 完成
# ============================================================
echo "======================================================================"
echo "           ✅ 環境設置完成！                                          "
echo "======================================================================"
echo ""
echo "📊 環境信息："
echo "   Rust 版本:    $(rustc --version)"
echo "   Cargo 版本:   $(cargo --version)"
echo "   VCPKG_ROOT:   $VCPKG_ROOT"
echo "   Triplet:      $TRIPLET"
echo ""
echo "🚀 下一步："
echo ""
echo "   1. 重新載入環境變量："
echo "      source $SHELL_RC"
echo ""
echo "   2. 構建 FamilyDesk:"
echo "      ./build-familydesk.sh"
echo ""
echo "   或直接運行（環境變量已在當前 shell 中設置）:"
echo "      export VCPKG_ROOT=$VCPKG_DIR"
echo "      ./build-familydesk.sh"
echo ""
echo "======================================================================"

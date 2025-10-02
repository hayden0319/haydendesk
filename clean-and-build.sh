#!/bin/bash

# FamilyDesk 完全清理並重新構建
# 清除所有緩存的構建配置

set -e

echo "🧹 完全清理構建環境..."
echo ""

# 1. 設置環境變量
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1️⃣  設置環境變量"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 設置 VCPKG_ROOT
if [ -z "$VCPKG_ROOT" ]; then
    if [ -d "$HOME/vcpkg" ]; then
        export VCPKG_ROOT="$HOME/vcpkg"
        echo "✅ 設置 VCPKG_ROOT=$VCPKG_ROOT"
    else
        echo "❌ 錯誤: vcpkg 未找到"
        echo "請先運行: ./setup-environment.sh"
        exit 1
    fi
else
    echo "✅ VCPKG_ROOT=$VCPKG_ROOT"
fi

# 檢測架構
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TRIPLET="arm64-osx"
    echo "✅ 檢測到 Apple Silicon (ARM64)"
else
    TRIPLET="x64-osx"
    echo "✅ 檢測到 Intel Mac (x86_64)"
fi

# 設置 PKG_CONFIG_PATH (macOS)
if command -v brew &> /dev/null; then
    BREW_PREFIX=$(brew --prefix)
    export PKG_CONFIG_PATH="$BREW_PREFIX/lib/pkgconfig:$BREW_PREFIX/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
    echo "✅ PKG_CONFIG_PATH 已設置"
fi

# 設置 Rust 環境
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "✅ Rust 環境已加載"
fi

# 檢查 vcpkg 是否已安裝依賴
VCPKG_INSTALLED="$VCPKG_ROOT/installed/$TRIPLET"
if [ ! -d "$VCPKG_INSTALLED" ]; then
    echo ""
    echo "⚠️  警告: vcpkg 依賴未安裝"
    echo "正在安裝 vcpkg 依賴（這可能需要較長時間）..."
    cd "$VCPKG_ROOT"
    ./vcpkg install --triplet $TRIPLET --manifest-root=/Users/hayden/Downloads/haydendesk
    cd /Users/hayden/Downloads/haydendesk
    echo "✅ vcpkg 依賴安裝完成"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2️⃣  清理緩存"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 清理 cargo 構建緩存
echo "🧹 清理 cargo 構建目錄..."
cargo clean

# 清理 cargo 註冊表緩存（可選，但會清除 GitHub Actions 路徑緩存）
echo "🧹 清理 cargo 註冊表緩存..."
rm -rf ~/.cargo/registry/index/*
rm -rf ~/.cargo/registry/cache/*
rm -rf ~/.cargo/git/checkouts/*

# 清理 target 目錄
echo "🧹 清理 target 目錄..."
rm -rf target

echo "✅ 清理完成"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3️⃣  驗證環境"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "📊 環境信息："
echo "   Rust 版本:      $(rustc --version)"
echo "   Cargo 版本:     $(cargo --version)"
echo "   VCPKG_ROOT:     $VCPKG_ROOT"
echo "   Triplet:        $TRIPLET"
echo "   vcpkg 依賴:     $VCPKG_INSTALLED"

# 驗證關鍵庫是否存在
if [ -f "$VCPKG_INSTALLED/lib/libopus.a" ]; then
    echo "   ✅ libopus 已安裝"
else
    echo "   ❌ libopus 未找到"
    echo ""
    echo "正在安裝 opus..."
    cd "$VCPKG_ROOT"
    ./vcpkg install opus:$TRIPLET
    cd /Users/hayden/Downloads/haydendesk
fi

if [ -f "$VCPKG_INSTALLED/lib/libvpx.a" ]; then
    echo "   ✅ libvpx 已安裝"
else
    echo "   ❌ libvpx 未找到"
fi

if [ -f "$VCPKG_INSTALLED/lib/libyuv.a" ]; then
    echo "   ✅ libyuv 已安裝"
else
    echo "   ❌ libyuv 未找到"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4️⃣  開始構建"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 構建
echo "🔨 構建 FamilyDesk（核心功能）..."
echo ""

# 設置額外的環境變量確保 vcpkg 正確檢測
export VCPKG_TRIPLET=$TRIPLET
export VCPKG_INSTALLED_DIR="$VCPKG_ROOT/installed"

cargo build --features family_desk --release

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ 構建完成！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📦 可執行文件位置: target/release/rustdesk"
echo ""
echo "🧪 測試運行："
echo "   RUST_LOG=info ./target/release/rustdesk"
echo ""

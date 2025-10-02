#!/bin/bash

# FamilyDesk 構建腳本 - 帶環境變量設置
# 如果環境變量未加載，此腳本會自動設置

set -e

echo "🔧 檢查環境變量..."

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

# 設置 PKG_CONFIG_PATH (macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    if command -v brew &> /dev/null; then
        BREW_PREFIX=$(brew --prefix)
        export PKG_CONFIG_PATH="$BREW_PREFIX/lib/pkgconfig:$BREW_PREFIX/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
        echo "✅ PKG_CONFIG_PATH 已設置"
    fi
fi

# 設置 Rust 環境
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "✅ Rust 環境已加載"
fi

echo ""
echo "🚀 開始構建 FamilyDesk..."
echo ""

# 檢查是否在正確的目錄
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 錯誤：請在項目根目錄運行此腳本"
    exit 1
fi

# 檢查 Rust 是否安裝
if ! command -v cargo &> /dev/null; then
    echo "❌ 錯誤：未找到 Rust/Cargo"
    echo "請先運行: ./setup-environment.sh"
    exit 1
fi

echo "📊 環境信息："
echo "   Rust 版本: $(rustc --version)"
echo "   Cargo 版本: $(cargo --version)"
echo "   VCPKG_ROOT: $VCPKG_ROOT"
echo ""

# 檢測架構
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TRIPLET="arm64-osx"
else
    TRIPLET="x64-osx"
fi

echo "🔨 構建 FamilyDesk（使用 triplet: $TRIPLET）..."
echo ""

# 清理之前的構建
echo "🧹 清理之前的構建..."
cargo clean

# 構建 FamilyDesk 核心功能
echo "🔨 開始編譯..."
cargo build --features family_desk --release

echo ""
echo "✅ 構建完成！"
echo "📦 可執行文件位置: target/release/rustdesk"
echo ""
echo "🧪 測試運行："
echo "   RUST_LOG=info ./target/release/rustdesk"

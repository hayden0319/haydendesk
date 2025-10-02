#!/bin/bash

# FamilyDesk 構建腳本
# 只構建必要的功能，避免不必要的依賴

set -e

echo "🚀 開始構建 FamilyDesk..."

# 檢查是否在正確的目錄
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 錯誤：請在項目根目錄運行此腳本"
    exit 1
fi

# 檢查 Rust 是否安裝
if ! command -v cargo &> /dev/null; then
    echo "❌ 錯誤：未找到 Rust/Cargo"
    echo ""
    echo "請先安裝 Rust："
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "安裝完成後，運行："
    echo "  source ~/.cargo/env"
    echo "  ./build-familydesk.sh"
    exit 1
fi

# 檢查 VCPKG_ROOT
if [ -z "$VCPKG_ROOT" ]; then
    echo "⚠️  警告：VCPKG_ROOT 未設置"
    echo ""
    echo "vcpkg 是必需的依賴。請先設置："
    echo "  git clone https://github.com/microsoft/vcpkg ~/vcpkg"
    echo "  cd ~/vcpkg"
    echo "  git checkout 120deac3062162151622ca4860575a33844ba10b"
    echo "  ./bootstrap-vcpkg.sh"
    echo "  export VCPKG_ROOT=~/vcpkg"
    echo ""
    read -p "是否繼續構建（可能會失敗）？[y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo "✅ 環境檢查通過"
echo "   Rust 版本: $(rustc --version)"
echo "   Cargo 版本: $(cargo --version)"
if [ -n "$VCPKG_ROOT" ]; then
    echo "   VCPKG_ROOT: $VCPKG_ROOT"
fi
echo ""

# 清理之前的構建
echo "🧹 清理之前的構建..."
cargo clean

# 構建 FamilyDesk 核心功能
# 不包含 flutter, hwcodec 等複雜功能
echo "🔨 構建 FamilyDesk（核心功能）..."
cargo build --features family_desk --release

echo "✅ 構建完成！"
echo "📦 可執行文件位置: target/release/rustdesk"
echo ""
echo "🧪 測試運行："
echo "   RUST_LOG=info ./target/release/rustdesk"

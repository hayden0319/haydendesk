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

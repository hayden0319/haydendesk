================================================================================
                    🎯 FamilyDesk - 開始之前必讀
================================================================================

⚠️  重要：GitHub Actions Workflow 選擇

  ✅ 正確: build-familydesk.yml
  ❌ 錯誤: flutter-tag.yml, playground.yml, flutter-build.yml

  詳細說明: WORKFLOW_QUICK_START.md

解決方案:
  ✅ 使用本地構建（推薦）- ./clean-and-build.sh
  ✅ 使用 build-familydesk.yml workflow

================================================================================

📦 快速開始 - 本地構建

  🆕 方法 1: 一鍵安裝（推薦，適合首次使用）

     cd /Users/hayden/Downloads/haydendesk
     ./setup-environment.sh
     # 此腳本會自動安裝：
     # - Rust
     # - 系統依賴（llvm, nasm等）
     # - vcpkg 及其依賴
     # - 配置環境變量

     然後運行：
     source ~/.bashrc  # 或 ~/.zshrc
     ./build-familydesk.sh

  方法 2: 手動構建（已有環境）

     cd /Users/hayden/Downloads/haydendesk
     ./build-familydesk.sh

  方法 3: 測試運行

     RUST_LOG=info ./target/release/rustdesk

================================================================================

📚 文檔索引

  新手入門:
    - README.md                      項目主頁
    - QUICKSTART.md                  5分鐘快速開始
    - QUICK_REFERENCE.md             快速參考卡

  技術文檔:
    - FAMILYDESK_README.md           完整技術文檔
    - IMPLEMENTATION_COMPLETE.md     實現詳情

  問題排查:
    - BUILD_TROUBLESHOOTING.md       構建問題（重要！）
    - FIXES_SUMMARY.md               修復總結
    - CODE_REVIEW_AND_IMPROVEMENTS.md 代碼審查

================================================================================

🔧 如果構建失敗

  1. 檢查 Rust 版本
     rustc --version
     # 需要 1.75+

  2. 檢查 vcpkg
     echo $VCPKG_ROOT
     # 應該指向 vcpkg 目錄

  3. 查看詳細錯誤
     詳見: BUILD_TROUBLESHOOTING.md

================================================================================

⚠️ 重要提示

  ✅ 正確的構建命令:
     cargo build --features family_desk --release

  ❌ 錯誤的構建命令:
     cargo build --features hwcodec,flutter,screencapturekit
     (這會導致缺少資源文件錯誤)

================================================================================

📞 需要幫助?

  1. 查看 BUILD_TROUBLESHOOTING.md
  2. 查看 QUICK_REFERENCE.md
  3. 檢查錯誤日誌

================================================================================

項目狀態: ✅ 代碼完整，準備本地構建
推薦操作: 運行 ./build-familydesk.sh

================================================================================

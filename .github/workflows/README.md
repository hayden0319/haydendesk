# GitHub Actions Workflows

## ✅ 唯一可用的 Workflow

### `build-familydesk.yml` - FamilyDesk 構建

**用途:** 構建 FamilyDesk macOS 版本

**特點:**
- ✅ 只構建 FamilyDesk 核心功能
- ✅ 使用當前分支代碼
- ✅ 包含所有必需依賴 (glib, gtk+3, cairo)
- ✅ 正確設置 PKG_CONFIG_PATH
- ✅ 使用 vcpkg manifest 模式
- ✅ 自動上傳構建產物

**觸發條件:**
- 手動觸發（workflow_dispatch）
- Push 到 main/master 分支
- Pull Request

**如何使用:**
1. 進入 GitHub 倉庫
2. 點擊 "Actions" 標籤
3. 選擇 "Build FamilyDesk"
4. 點擊 "Run workflow"
5. 選擇分支
6. 點擊綠色的 "Run workflow" 按鈕
7. 等待構建完成（約 15-30 分鐘）
8. 下載構建產物

**構建產物:**
- `familydesk-macos-1.4.2` - macOS 可執行文件

---

## 🗑️ 已刪除的 Workflows

以下 workflows 已被刪除，因為它們不適用於 FamilyDesk:

- ❌ `playground.yml` - 使用舊代碼，錯誤功能
- ❌ `flutter-tag.yml` - Flutter UI 版本
- ❌ `flutter-build.yml` - Flutter UI 構建
- ❌ `flutter-nightly.yml` - Flutter 夜間構建
- ❌ `flutter-ci.yml` - Flutter CI
- ❌ `ci.yml` - CI 測試
- ❌ `bridge.yml` - 不相關
- ❌ `fdroid.yml` - F-Droid 構建
- ❌ `winget.yml` - Windows 包管理器
- ❌ `third-party-RustDeskTempTopMostWindow.yml` - 第三方工具

**現在倉庫中只有:**
- ✅ `build-familydesk.yml` - FamilyDesk 構建（唯一需要的）
- ✅ `clear-cache.yml` - 清除緩存工具

---

## 🚀 快速開始

### GitHub 網頁界面

```
1. 訪問: https://github.com/[用戶名]/haydendesk/actions
2. 左側選擇: "Build FamilyDesk"
3. 右側點擊: "Run workflow"
4. 選擇分支: main
5. 點擊: "Run workflow" (綠色按鈕)
6. 等待完成
7. 下載 Artifacts
```

### GitHub CLI

```bash
# 觸發構建
gh workflow run build-familydesk.yml

# 查看狀態
gh run list --workflow=build-familydesk.yml

# 監控運行
gh run watch

# 下載產物
gh run download
```

---

## 🔍 構建步驟說明

`build-familydesk.yml` 執行以下步驟:

1. **Checkout source code** - 檢出代碼和子模塊
2. **Install Rust toolchain** - 安裝 Rust 1.75
3. **Install build dependencies** - 安裝 glib, gtk+3, cairo, pango, atk, gdk-pixbuf
4. **Setup vcpkg** - 設置 vcpkg (版本: 120deac3062162151622ca4860575a33844ba10b)
5. **Set PKG_CONFIG_PATH** - 設置環境變量指向 brew 庫
6. **Verify pkg-config setup** - 驗證 glib-2.0, gtk+-3.0, cairo 可用
7. **Install vcpkg dependencies** - 使用 manifest 模式安裝 opus, vpx, yuv 等
8. **Build FamilyDesk** - 執行 `cargo build --features family_desk --release`
9. **Verify build** - 驗證生成的可執行文件
10. **Upload artifact** - 上傳構建產物

---

## 📊 本地構建 vs GitHub Actions

| 特性 | 本地構建 | GitHub Actions |
|------|---------|---------------|
| **速度** | 快（如果已有依賴） | 慢（每次安裝依賴） |
| **環境** | 需要自己設置 | 自動設置 |
| **調試** | 容易 | 需要查看日誌 |
| **適用場景** | 開發測試 | 發布構建 |
| **配額** | 無限制 | 有月度限制 |

**推薦流程:**
1. 本地開發和測試 (`./clean-and-build.sh`)
2. 確認可用後推送到 GitHub
3. 使用 GitHub Actions 創建發布版本

---

## 🔧 故障排除

### 常見問題

**Q: Workflow 找不到？**
A: 確保已推送最新代碼到 GitHub，檢查 `.github/workflows/build-familydesk.yml` 文件存在

**Q: 構建失敗 - glib-2.0 找不到？**
A: 檢查 "Verify pkg-config setup" 步驟，應該顯示所有庫都找到

**Q: 構建失敗 - opus 相關錯誤？**
A: 檢查 "Install vcpkg dependencies" 步驟，確保使用 manifest 模式

**Q: 構建時間太長？**
A: 第一次構建需要編譯 vcpkg 依賴（約 15-30 分鐘），後續構建會快很多

**Q: 下載的產物在哪？**
A: 在 workflow 運行完成後，滾動到頁面底部 "Artifacts" 部分

---

## 📝 修改 Workflow

如果需要修改構建配置，編輯 `build-familydesk.yml`:

### 修改依賴
```yaml
- name: Install build dependencies
  run: |
    brew install llvm nasm pkg-config glib gtk+3 cairo [新依賴]
```

### 修改 Feature Flags
```yaml
- name: Build FamilyDesk
  run: |
    cargo build --features family_desk,your_feature --release
```

### 修改環境變量
```yaml
env:
  RUST_VERSION: "1.75"
  VERSION: "1.4.2"
  # 添加更多變量...
```

---

## 📚 相關文檔

- [WORKFLOW_QUICK_START.md](../../WORKFLOW_QUICK_START.md) - 快速開始指南
- [GITHUB_ACTIONS_USAGE.md](../../GITHUB_ACTIONS_USAGE.md) - 詳細使用說明
- [BUILD_TROUBLESHOOTING.md](../../BUILD_TROUBLESHOOTING.md) - 構建問題排查
- [VCPKG_SETUP.md](../../VCPKG_SETUP.md) - vcpkg 設置指南

---

**狀態:** ✅ 簡化完成 - 只保留必要的 workflow
**最後更新:** 2025-02-10
**推薦:** 使用 `build-familydesk.yml` 或本地構建

# GitHub Actions 快速開始

## ✅ 正確的構建方式

### 方法 1: GitHub 網頁界面（推薦）

1. 訪問 Actions 頁面
   ```
   https://github.com/[你的用戶名]/haydendesk/actions
   ```

2. 在左側選擇 **"Build FamilyDesk"**

3. 點擊右側的 **"Run workflow"** 下拉按鈕

4. 選擇分支（通常是 `main` 或 `master`）

5. 點擊綠色的 **"Run workflow"** 按鈕

6. 等待構建完成（約 15-30 分鐘）

7. 下載構建產物
   - 點擊完成的 workflow run
   - 滾動到底部 "Artifacts" 部分
   - 下載 `familydesk-macos-1.4.2`

---

### 方法 2: GitHub CLI（命令行）

```bash
# 安裝 GitHub CLI（如果未安裝）
brew install gh

# 登錄
gh auth login

# 觸發構建
gh workflow run build-familydesk.yml

# 查看運行狀態
gh run list --workflow=build-familydesk.yml

# 查看最新運行的詳細信息
gh run view

# 下載構建產物
gh run download
```

---

## ❌ 錯誤的構建方式

### 不要使用這些 Workflows：

| Workflow | 為什麼不要用 |
|----------|------------|
| `flutter-tag.yml` | 構建 Flutter UI 版本，不是 FamilyDesk |
| `flutter-build.yml` | 需要 Flutter SDK 和額外資源 |
| `flutter-nightly.yml` | Flutter 夜間構建 |
| `playground.yml` | 已禁用，使用舊代碼和錯誤功能 |
| `ci.yml` | CI 測試，不是完整構建 |

### 常見錯誤

❌ **錯誤：** 運行 `flutter-tag.yml`
```
錯誤信息: mac-tray-dark-x2.png 找不到
原因: 嘗試構建 Flutter 功能
```

❌ **錯誤：** 運行 `playground.yml`
```
錯誤信息: 已過時的 workflow
原因: 使用舊代碼和錯誤的 feature flags
```

✅ **正確：** 運行 `build-familydesk.yml`
```
結果: 成功構建 FamilyDesk
功能: 只包含核心 FamilyDesk 功能
```

---

## 🔍 如何識別正確的 Workflow

在 GitHub Actions 頁面，查找：

✅ **Workflow 名稱：** "Build FamilyDesk"
✅ **文件名：** `build-familydesk.yml`
✅ **描述：** Build FamilyDesk for macOS

避免：

❌ **名稱包含：** "Flutter", "Nightly", "Playground", "CI"
❌ **描述提到：** Flutter UI, Multi-version, Legacy

---

## 📊 構建狀態檢查

### 查看構建進度

1. 點擊運行中的 workflow
2. 查看每個步驟的狀態：
   - ✅ Checkout source code
   - ✅ Install Rust toolchain
   - ✅ Install build dependencies
   - ✅ Setup vcpkg
   - ✅ Set PKG_CONFIG_PATH
   - ✅ Verify pkg-config setup ← **重要！確保通過**
   - ✅ Install vcpkg dependencies
   - ✅ Build FamilyDesk ← **最重要！**
   - ✅ Upload artifact

### 關鍵檢查點

**步驟 "Verify pkg-config setup"** 應該顯示：
```
✅ glib-2.0 found (version: 2.82.x)
✅ gtk+-3.0 found (version: 3.x.x)
✅ cairo found (version: 1.x.x)
```

**步驟 "Build FamilyDesk"** 應該顯示：
```
cargo build --features family_desk --release
```

如果看到 `--features hwcodec,flutter` 則說明運行了錯誤的 workflow！

---

## 🚀 完整流程示例

### 首次構建

```bash
# 1. Clone 倉庫（如果還沒有）
git clone https://github.com/[你的用戶名]/haydendesk.git
cd haydendesk

# 2. 確保最新代碼已推送
git add .
git commit -m "Update workflows"
git push

# 3. 觸發構建（使用 GitHub CLI）
gh workflow run build-familydesk.yml

# 4. 監控構建
gh run watch

# 5. 下載結果
gh run download
```

### 使用網頁界面

```
1. 訪問: https://github.com/[用戶名]/haydendesk/actions
2. 左側選擇: "Build FamilyDesk"
3. 右側點擊: "Run workflow"
4. 選擇分支: main
5. 點擊: "Run workflow" (綠色按鈕)
6. 等待完成
7. 下載 Artifacts
```

---

## 💡 提示

1. **首次構建較慢**
   - vcpkg 需要編譯依賴（約 10-15 分鐘）
   - 後續構建會使用緩存（快很多）

2. **檢查日誌**
   - 如果失敗，展開失敗的步驟
   - 查看完整錯誤信息
   - 參考 [BUILD_TROUBLESHOOTING.md](BUILD_TROUBLESHOOTING.md)

3. **本地測試優先**
   - 使用 `./clean-and-build.sh` 本地構建
   - 本地成功後再用 GitHub Actions
   - 節省 Actions 分鐘數

4. **構建產物位置**
   - 在 workflow run 頁面底部
   - "Artifacts" 部分
   - 文件名: `familydesk-macos-1.4.2`
   - 解壓後是 `rustdesk` 可執行文件

---

## 📞 需要幫助？

如果構建失敗，按順序檢查：

1. ✅ 確認使用的是 `build-familydesk.yml`
2. ✅ 查看 "Verify pkg-config setup" 步驟
3. ✅ 查看完整錯誤日誌
4. ✅ 參考 [GITHUB_ACTIONS_USAGE.md](GITHUB_ACTIONS_USAGE.md)
5. ✅ 參考 [BUILD_TROUBLESHOOTING.md](BUILD_TROUBLESHOOTING.md)

---

**記住：只使用 `build-familydesk.yml` ！** ✅

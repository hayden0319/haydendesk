# GitHub Actions 使用指南

## ⚠️ 重要：正確的 Workflow 選擇

### ✅ 使用這個 Workflow：

**`build-familydesk.yml`** - FamilyDesk 專用構建

- ✅ 只構建 FamilyDesk 核心功能
- ✅ 使用正確的 feature flags: `family_desk`
- ✅ 包含所有必需依賴 (glib, gtk+3, cairo)
- ✅ 正確設置 PKG_CONFIG_PATH
- ✅ 使用 vcpkg manifest 模式

**如何使用：**

1. 進入 GitHub 倉庫
2. 點擊 "Actions" 標籤
3. 選擇 "Build FamilyDesk" workflow
4. 點擊 "Run workflow"
5. 選擇分支（通常是 main 或 master）
6. 點擊綠色的 "Run workflow" 按鈕

### ❌ 不要使用這些 Workflows：

**`playground.yml.disabled`** (已禁用)
- ❌ 使用舊的代碼版本
- ❌ 嘗試構建不需要的功能 (hwcodec, flutter, screencapturekit)
- ❌ 缺少必需的資源文件 (mac-tray-dark-x2.png)
- ❌ 會導致編譯失敗

**`flutter-build.yml`**
- ❌ 用於構建 Flutter UI 版本，不是 FamilyDesk
- ❌ 需要額外的 Flutter 依賴和資源

**`ci.yml`**
- ❌ 用於持續集成測試，不是完整構建

---

## 📊 Workflow 比較

| 特性 | build-familydesk.yml | playground.yml | flutter-build.yml |
|------|---------------------|----------------|-------------------|
| **用途** | FamilyDesk 構建 | 多版本測試（已棄用） | Flutter UI 構建 |
| **Feature Flags** | `family_desk` | `hwcodec,flutter,...` | `flutter,hwcodec,...` |
| **依賴** | glib, gtk+3, vcpkg | 不完整 | Flutter SDK, vcpkg |
| **資源文件** | ✅ 完整 | ❌ 缺少 | ✅ 完整 |
| **PKG_CONFIG_PATH** | ✅ 已設置 | ❌ 未設置 | 部分設置 |
| **狀態** | ✅ 活躍 | ❌ 已禁用 | ⚠️  僅用於 Flutter |
| **推薦使用** | ✅ 是 | ❌ 否 | ❌ 否 |

---

## 🔧 build-familydesk.yml 詳細說明

### 構建步驟

1. **Checkout source code** - 檢出代碼和子模塊
2. **Install Rust toolchain** - 安裝 Rust 1.75
3. **Install build dependencies** - 安裝系統依賴
   ```bash
   brew install llvm nasm pkg-config glib gtk+3 cairo pango atk gdk-pixbuf
   ```
4. **Setup vcpkg** - 設置 vcpkg 並檢出正確版本
   ```bash
   git checkout 120deac3062162151622ca4860575a33844ba10b
   ```
5. **Set PKG_CONFIG_PATH** - 設置環境變量
   ```bash
   PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/glib/lib/pkgconfig:...
   ```
6. **Verify pkg-config setup** - 驗證庫可用
7. **Install vcpkg dependencies** - 使用 manifest 模式安裝
   ```bash
   vcpkg install --triplet x64-osx
   ```
8. **Build FamilyDesk** - 構建核心功能
   ```bash
   cargo build --features family_desk --release
   ```
9. **Upload artifact** - 上傳構建結果

### 環境變量

```yaml
RUST_VERSION: "1.75"
VERSION: "1.4.2"
RS_PUB_KEY: "iQX+6aUbS40CL9jJElm4jSMs8aKzePqMsFH1HACDI5E="
RENDEZVOUS_SERVER: "nas.haydenstudio.hk"
API_SERVER: "http://nas.haydenstudio.hk:21114"
```

### 觸發條件

- ✅ 手動觸發 (workflow_dispatch)
- ✅ Push 到 main/master 分支
- ✅ Pull request

---

## 🚨 常見錯誤

### 錯誤 1: `mac-tray-dark-x2.png` 找不到

**原因：** 使用了錯誤的 workflow (playground.yml)

**解決：** 使用 `build-familydesk.yml`

### 錯誤 2: `glib-2.0` 找不到

**原因：** PKG_CONFIG_PATH 未設置

**解決：**
- 確保使用最新的 `build-familydesk.yml`
- 檢查 "Set PKG_CONFIG_PATH" 步驟是否執行成功

### 錯誤 3: vcpkg manifest mode 錯誤

**原因：** vcpkg 版本不正確或使用了傳統模式命令

**解決：**
- 確保 vcpkg checkout 到正確版本: `120deac3062162151622ca4860575a33844ba10b`
- 使用 `vcpkg install --triplet x64-osx` (不要指定包名)

### 錯誤 4: `opus/opus_multistream.h` 找不到

**原因：** vcpkg 依賴未安裝或路徑錯誤

**解決：**
- 檢查 "Install vcpkg dependencies" 步驟
- 確保 VCPKG_ROOT 環境變量正確設置

---

## 📝 修改 Workflow

如果需要修改構建配置：

1. **修改依賴：**
   編輯步驟 "Install build dependencies"
   ```yaml
   - name: Install build dependencies
     run: |
       brew install [你的依賴]
   ```

2. **修改 Feature Flags：**
   編輯步驟 "Build FamilyDesk"
   ```yaml
   - name: Build FamilyDesk
     run: |
       cargo build --features family_desk --release
   ```

3. **修改環境變量：**
   編輯 workflow 頂部的 `env:` 部分

---

## ✅ 驗證清單

在運行 workflow 之前，確保：

- [ ] 使用 `build-familydesk.yml` workflow
- [ ] 沒有手動觸發 `playground.yml`（已禁用）
- [ ] 代碼已 push 到 GitHub
- [ ] 選擇正確的分支

在 workflow 運行期間，檢查：

- [ ] "Install build dependencies" 成功安裝 glib, gtk+3 等
- [ ] "Verify pkg-config setup" 顯示所有庫都找到
- [ ] "Install vcpkg dependencies" 成功安裝 opus, vpx, yuv
- [ ] "Build FamilyDesk" 使用 `--features family_desk`

---

## 🎯 最佳實踐

1. **始終使用 build-familydesk.yml**
   - 這是專為 FamilyDesk 設計的 workflow
   - 包含所有必需配置

2. **檢查日誌**
   - 展開每個步驟查看詳細輸出
   - 特別注意 "Verify pkg-config setup" 步驟

3. **本地測試優先**
   - 使用 `./clean-and-build.sh` 在本地構建
   - 本地成功後再使用 GitHub Actions

4. **保持 vcpkg 版本一致**
   - GitHub Actions 使用 `120deac3062162151622ca4860575a33844ba10b`
   - 本地也應使用相同版本

---

## 📚 相關文檔

- [BUILD_TROUBLESHOOTING.md](BUILD_TROUBLESHOOTING.md) - 構建問題排查
- [VCPKG_SETUP.md](VCPKG_SETUP.md) - vcpkg 設置指南
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - 快速參考
- [README_FIRST.txt](README_FIRST.txt) - 開始之前必讀

---

**最後更新：** 2025-02-10
**狀態：** ✅ build-familydesk.yml 可用，playground.yml 已禁用

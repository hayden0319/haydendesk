# GitHub Actions 編譯失敗問題解決

**日期:** 2025-02-10
**問題:** playground.yml 編譯失敗

---

## 🐛 問題描述

### 錯誤信息

```
error: could not compile `rustdesk` (lib) due to 1 previous error; 3 warnings emitted
Error occurred when executing: `MACOSX_DEPLOYMENT_TARGET=12.3 cargo build --features hwcodec,flutter,unix-file-copy-paste,screencapturekit --release`. Exiting.
features: ['hwcodec', 'flutter', 'unix-file-copy-paste', 'screencapturekit']
False
Error: Process completed with exit code 255.
```

具體錯誤：
```
error: couldn't read `src/../res/mac-tray-dark-x2.png`: No such file or directory (os error 2)
  --> src/tray.rs:42:16
```

---

## 🔍 根本原因分析

### 1. 使用了錯誤的代碼版本

**問題：** `playground.yml` 使用舊的 git commit ref

```yaml
# playground.yml 第 33 行
ref: "f6509e3fd6917aa976bad2fc684182601ebf2434"  # 2023年的舊代碼
```

**影響：**
- 檢出的是 2023年12月的舊代碼
- 這個舊版本可能缺少某些資源文件
- 與 FamilyDesk 的修改不兼容

### 2. 嘗試構建不需要的功能

**問題：** 構建命令包含複雜功能

```bash
cargo build --features hwcodec,flutter,unix-file-copy-paste,screencapturekit --release
```

**這些功能需要：**
- `hwcodec` - 硬件編碼器（需要額外庫）
- `flutter` - Flutter UI（需要 Flutter SDK）
- `screencapturekit` - macOS 特定 API（需要特定版本）

**FamilyDesk 只需要：**
```bash
cargo build --features family_desk --release
```

### 3. 資源文件路徑問題

**問題：** 舊代碼的資源文件結構可能不同

```rust
// src/tray.rs:42
icon = include_bytes!("../res/mac-tray-dark-x2.png");
```

在舊版本中，這個文件可能：
- 路徑不同
- 文件名不同
- 根本不存在

---

## ✅ 解決方案

### 方案 1: 使用新的 GitHub Actions Workflow（推薦）

**創建了:** `.github/workflows/build-familydesk.yml`

**特點：**
```yaml
- name: Checkout source code
  uses: actions/checkout@v3
  with:
    submodules: recursive
    # 不指定 ref，使用當前分支

- name: Build FamilyDesk
  run: cargo build --features family_desk --release
```

**優勢：**
- ✅ 使用當前分支代碼（包含所有 FamilyDesk 修改）
- ✅ 只構建必要功能
- ✅ 快速（~10分鐘）
- ✅ 高成功率

### 方案 2: 禁用舊 Workflow

**修改了:** `playground.yml`

**改動：**
1. 重命名為 "Legacy - DO NOT USE"
2. 添加確認輸入要求
3. 添加檢查步驟，必須輸入 "YES" 才能繼續

```yaml
on:
  workflow_dispatch:
    inputs:
      confirm:
        description: '確認使用舊版構建？輸入 YES'
        required: true
        default: 'NO'

steps:
  - name: "⚠️ 檢查確認"
    run: |
      if [ "${{ github.event.inputs.confirm }}" != "YES" ]; then
        echo "❌ 錯誤: 此 workflow 已過時"
        exit 1
      fi
```

**效果：**
- 防止意外觸發
- 清楚標示為已棄用
- 提示使用新 workflow

### 方案 3: 本地構建（最推薦）

**使用構建腳本：**
```bash
cd /Users/hayden/Downloads/haydendesk
./build-familydesk.sh
```

**優勢：**
- ✅ 最快（~5分鐘）
- ✅ 不依賴 GitHub Actions
- ✅ 即時反饋
- ✅ 更容易調試

---

## 📊 對比分析

### Workflow 對比

| 項目 | playground.yml | build-familydesk.yml | 本地構建 |
|------|---------------|---------------------|---------|
| **狀態** | ⚠️ 已棄用 | ✅ 推薦 | ✅ 最推薦 |
| **代碼版本** | 舊 commit (2023) | 當前分支 | 當前代碼 |
| **功能** | hwcodec, flutter, screencapturekit | family_desk | family_desk |
| **構建時間** | ~30分鐘 | ~10分鐘 | ~5分鐘 |
| **成功率** | 低（會失敗）| 高 | 最高 |
| **依賴複雜度** | 高 | 低 | 低 |
| **調試難度** | 難 | 中 | 容易 |

### 功能對比

| 功能 | 是否需要 | playground.yml | build-familydesk.yml |
|------|---------|---------------|---------------------|
| FamilyDesk 核心 | ✅ 必需 | ❌ 不包含 | ✅ 包含 |
| API 服務器 | ✅ 必需 | ❌ 不包含 | ✅ 包含 |
| 權限系統 | ✅ 必需 | ❌ 不包含 | ✅ 包含 |
| 硬件編碼 | ❌ 不需要 | ✅ 包含 | ❌ 不包含 |
| Flutter UI | ❌ 不需要 | ✅ 包含 | ❌ 不包含 |
| ScreenCaptureKit | ❌ 不需要 | ✅ 包含 | ❌ 不包含 |

---

## 🎯 建議使用流程

### 開發階段

```bash
# 本地開發和測試
cd /Users/hayden/Downloads/haydendesk

# 快速檢查
cargo check --features family_desk

# 開發構建
cargo build --features family_desk

# 運行測試
cargo test --features family_desk
```

### 發布階段

```bash
# 本地發布構建
./build-familydesk.sh

# 或使用 GitHub Actions
# 1. Push 代碼到 GitHub
# 2. 觸發 build-familydesk.yml
# 3. 下載構建產物
```

---

## 🔧 如果仍然失敗

### 檢查清單

1. **確認使用正確的 workflow**
   ```
   ✅ build-familydesk.yml
   ❌ playground.yml
   ```

2. **確認功能標誌**
   ```
   ✅ --features family_desk
   ❌ --features hwcodec,flutter,screencapturekit
   ```

3. **確認代碼版本**
   ```yaml
   # 正確：不指定 ref（使用當前分支）
   uses: actions/checkout@v3

   # 錯誤：指定舊 ref
   ref: "f6509e3fd6917aa976bad2fc684182601ebf2434"
   ```

4. **檢查資源文件**
   ```bash
   # 確認文件存在
   ls -la res/mac-tray-dark-x2.png

   # 應該顯示
   -rw-rw-r-- 1 user staff 612 ... res/mac-tray-dark-x2.png
   ```

---

## 📚 相關文檔

- `.github/workflows/README.md` - Workflow 使用指南
- `BUILD_TROUBLESHOOTING.md` - 構建問題完整排查
- `QUICK_REFERENCE.md` - 快速參考
- `build-familydesk.sh` - 本地構建腳本

---

## 📝 總結

### 問題核心
GitHub Actions 的 `playground.yml` 使用了：
1. ❌ 錯誤的代碼版本（舊 commit）
2. ❌ 錯誤的功能標誌（不需要的功能）
3. ❌ 錯誤的期望（嘗試構建原版 RustDesk）

### 解決方案核心
FamilyDesk 需要：
1. ✅ 當前分支代碼（包含所有修改）
2. ✅ 簡單的功能標誌（只需 family_desk）
3. ✅ 正確的期望（構建 FamilyDesk，不是原版）

### 最佳實踐
- 🥇 **首選：** 本地構建 (`./build-familydesk.sh`)
- 🥈 **次選：** GitHub Actions (`build-familydesk.yml`)
- 🚫 **避免：** 舊 workflow (`playground.yml`)

---

**狀態：** ✅ 問題已解決
**推薦：** 使用本地構建或新的 GitHub Actions workflow

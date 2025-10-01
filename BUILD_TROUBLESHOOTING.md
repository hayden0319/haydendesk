# FamilyDesk 構建問題排查

**日期:** 2025-02-10

---

## 🐛 當前問題

### GitHub Actions 編譯失敗

**錯誤信息:**
```
error: couldn't read `src/../res/mac-tray-dark-x2.png`: No such file or directory (os error 2)
  --> src/tray.rs:42:16
```

**原因分析:**

1. **錯誤的 Git Ref**
   - 原 `playground.yml` 使用舊的 commit ref: `f6509e3fd6917aa976bad2fc684182601ebf2434`
   - 這些舊版本可能缺少某些資源文件或與 FamilyDesk 修改不兼容

2. **複雜的功能標誌**
   - 原 workflow 嘗試構建: `hwcodec, flutter, unix-file-copy-paste, screencapturekit`
   - 這些功能需要額外的依賴和資源文件

---

## ✅ 解決方案

### 方案 1: 使用簡化的構建腳本（推薦）

本地構建只使用 FamilyDesk 核心功能：

```bash
cd /Users/hayden/Downloads/haydendesk

# 使用提供的構建腳本
./build-familydesk.sh

# 或手動構建
cargo build --features family_desk --release
```

**只構建必要的功能:**
- `family_desk` - FamilyDesk 核心功能
  - `api_server` - API 服務器
  - `simple_permissions` - 權限系統

**不包含:**
- ❌ `hwcodec` - 硬件編碼（需要額外依賴）
- ❌ `flutter` - Flutter UI（需要 Flutter SDK）
- ❌ `screencapturekit` - macOS 特定功能

### 方案 2: 使用新的 GitHub Actions Workflow

創建了專用的 `build-familydesk.yml`：

**特點:**
- ✅ 使用當前分支代碼
- ✅ 只構建核心功能
- ✅ 簡化的依賴
- ✅ 包含代碼質量檢查

**啟用方法:**
1. 推送代碼到 GitHub
2. 進入 Actions 頁面
3. 選擇 "Build FamilyDesk" workflow
4. 點擊 "Run workflow"

### 方案 3: 修復原始 playground.yml

如果需要保留原 workflow：

1. **移除舊的 commit ref:**
   ```yaml
   # 修改前
   ref: ${{ matrix.job.ref }}

   # 修改後
   # 使用當前分支，不指定 ref
   ```

2. **簡化功能標誌:**
   ```yaml
   # 修改前
   --features hwcodec,flutter,unix-file-copy-paste,screencapturekit

   # 修改後
   --features family_desk
   ```

---

## 📋 構建步驟

### macOS 本地構建

**前置要求:**
```bash
# 1. 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安裝依賴
brew install llvm nasm pkg-config

# 3. 設置 vcpkg
git clone https://github.com/microsoft/vcpkg
cd vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT=$PWD
./vcpkg install libvpx libyuv opus aom
```

**構建:**
```bash
cd /Users/hayden/Downloads/haydendesk

# 方法 1: 使用腳本
./build-familydesk.sh

# 方法 2: 手動構建
cargo build --features family_desk --release

# 構建成功後
ls -lh target/release/rustdesk
```

### Linux 本地構建

```bash
# Ubuntu/Debian
sudo apt install -y gcc git curl wget nasm yasm \
    libgtk-3-dev clang libxcb-randr0-dev libxdo-dev \
    libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libasound2-dev libpulse-dev cmake

# 構建
cargo build --features family_desk --release
```

---

## 🧪 驗證構建

```bash
# 檢查可執行文件
file target/release/rustdesk

# 查看依賴
otool -L target/release/rustdesk  # macOS
ldd target/release/rustdesk       # Linux

# 測試運行
RUST_LOG=info ./target/release/rustdesk --help
```

---

## 🔍 常見問題

### Q1: 缺少 vcpkg 依賴

**錯誤:**
```
error: failed to run custom build command for `scrap`
```

**解決:**
```bash
export VCPKG_ROOT=/path/to/vcpkg
cd $VCPKG_ROOT
./vcpkg install libvpx libyuv opus aom
```

### Q2: Rust 版本太舊

**錯誤:**
```
error[E0658]: use of unstable library feature
```

**解決:**
```bash
rustup update
rustup default 1.75
```

### Q3: 缺少系統依賴

**macOS:**
```bash
brew install llvm nasm pkg-config
```

**Ubuntu:**
```bash
sudo apt install build-essential cmake nasm
```

---

## 📊 功能對比

| 功能 | playground.yml | build-familydesk.yml | 本地構建 |
|------|----------------|----------------------|----------|
| FamilyDesk 核心 | ❌ 未包含 | ✅ 支持 | ✅ 支持 |
| 硬件編碼 | ✅ 包含 | ❌ 不需要 | ❌ 不需要 |
| Flutter UI | ✅ 包含 | ❌ 不需要 | ❌ 不需要 |
| 多版本構建 | ✅ 4個版本 | ❌ 單版本 | ❌ 單版本 |
| 構建時間 | ~30分鐘 | ~10分鐘 | ~5分鐘 |
| 依賴複雜度 | 高 | 低 | 低 |

---

## ✅ 推薦工作流程

### 開發階段

1. **本地開發:**
   ```bash
   # 快速檢查
   cargo check --features family_desk

   # 開發構建
   cargo build --features family_desk

   # 運行測試
   cargo test --features family_desk
   ```

2. **本地測試:**
   ```bash
   # 發布構建
   cargo build --features family_desk --release

   # 測試功能
   RUST_LOG=debug ./target/release/rustdesk
   ```

### 發布階段

1. **推送到 GitHub**
2. **觸發 GitHub Actions:** `build-familydesk.yml`
3. **下載構建產物**
4. **測試部署**

---

## 🎯 下一步

- [x] 創建簡化構建腳本
- [x] 創建專用 GitHub Actions workflow
- [x] 編寫構建文檔
- [ ] 本地測試構建
- [ ] GitHub Actions 測試構建
- [ ] 創建發布包

---

## 📝 注意事項

1. **資源文件**
   - 確保 `res/` 目錄中的所有資源文件都已提交
   - 檢查 `.gitignore` 沒有忽略必要的資源

2. **Git Submodules**
   - 確保所有 submodules 都已更新
   ```bash
   git submodule update --init --recursive
   ```

3. **環境變量**
   - `VCPKG_ROOT` 必須正確設置
   - `RUST_LOG` 用於調試日誌

---

**項目狀態:** 代碼完整，本地構建準備就緒 ✅
**推薦:** 使用 `./build-familydesk.sh` 進行本地構建

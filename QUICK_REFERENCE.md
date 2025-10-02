# FamilyDesk 快速參考

**項目:** FamilyDesk - RustDesk 家庭支援分支
**目錄:** `/Users/hayden/Downloads/haydendesk`

---

## ⚡ 快速命令

### 構建
```bash
# 推薦方式
./build-familydesk.sh

# 手動構建
cargo build --features family_desk --release
```

### 運行
```bash
# 帶日誌
RUST_LOG=info ./target/release/rustdesk

# API 服務器
cargo run --features family_desk --bin api_server
```

### 測試
```bash
# 檢查代碼
cargo check --features family_desk

# 運行測試
cargo test --features family_desk
```

---

## 📁 重要文件

| 文件 | 用途 |
|------|------|
| `README.md` | 項目主文檔 |
| `FAMILYDESK_README.md` | 完整技術文檔 |
| `QUICKSTART.md` | 5分鐘快速開始 |
| `BUILD_TROUBLESHOOTING.md` | 構建問題排查 |
| `FIXES_SUMMARY.md` | 修復總結 |
| `IMPLEMENTATION_COMPLETE.md` | 實現詳情 |

---

## 🔑 核心功能

### API 服務器
- **地址:** `http://nas.haydenstudio.hk:21114`
- **默認帳號:** admin / admin123
- **主要服務器:** nas.haydenstudio.hk
- **備用服務器:** 5個（需配置）

### 權限系統
- ✅ 滑鼠控制
- ✅ 鍵盤控制
- ✅ 剪貼板
- ✅ 文件傳輸
- ✅ 音頻（規劃中）

---

## 🛠️ 配置位置

### 備用服務器
```
src/api_server_config.rs (第 54-83 行)
```

### 權限設置
```
~/.config/rustdesk/permissions.json
```

### 服務器配置
```
libs/hbb_common/src/config.rs
```

---

## 🐛 常見錯誤

### 錯誤 1: vcpkg 未找到
```bash
export VCPKG_ROOT=/path/to/vcpkg
```

### 錯誤 2: 資源文件缺失
```bash
# 確保使用正確的功能標誌
cargo build --features family_desk  # ✅
# 不要用
cargo build --features hwcodec,flutter  # ❌
```

### 錯誤 3: Rust 版本
```bash
rustup update
rustup default 1.75
```

---

## 📞 API 端點

### 登錄
```bash
curl -X POST http://localhost:21114/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123","device_id":"123"}'
```

### 設置權限
```bash
curl -X POST http://localhost:21114/api/set-permission \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123","device_id":"123","permission_name":"allow_mouse","value":false}'
```

### 創建帳號
```bash
curl -X POST http://localhost:21114/api/create-account \
  -H "Content-Type: application/json" \
  -d '{"username":"user1","password":"pass123","role":"family","can_modify_settings":false,"device_ids":["123"]}'
```

---

## ✅ 驗證清單

### 構建前
- [ ] 已安裝 Rust 1.75+
- [ ] 已設置 VCPKG_ROOT
- [ ] 已安裝 vcpkg 依賴（libvpx, libyuv, opus, aom）
- [ ] 已安裝系統依賴（llvm, nasm, pkg-config）

### 構建後
- [ ] 可執行文件存在: `target/release/rustdesk`
- [ ] 文件大小合理（> 50MB）
- [ ] 能成功運行 `./target/release/rustdesk --help`

### 測試
- [ ] API 服務器啟動成功
- [ ] 權限設置生效
- [ ] 滑鼠/鍵盤阻斷測試通過
- [ ] 日誌顯示 🚫 阻斷信息

---

## 🚀 下一步

1. **首次構建**
   ```bash
   ./build-familydesk.sh
   ```

2. **啟動服務器**
   ```bash
   RUST_LOG=info cargo run --features family_desk --bin api_server
   ```

3. **測試權限**
   - 禁用滑鼠
   - 嘗試遠程控制
   - 檢查日誌

4. **配置備用服務器**
   - 編輯 `src/api_server_config.rs`
   - 啟用 5 個備用服務器

---

**快速幫助:** 如有問題，查看 `BUILD_TROUBLESHOOTING.md`

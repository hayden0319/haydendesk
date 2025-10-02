# GitHub Actions Workflows

## ✅ 推薦使用

### `build-familydesk.yml` - FamilyDesk 構建 (推薦)

**用途:** 構建 FamilyDesk 核心功能

**特點:**
- ✅ 只構建必要的功能
- ✅ 使用當前分支代碼
- ✅ 快速構建（~10分鐘）
- ✅ 包含代碼質量檢查
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
5. 等待構建完成
6. 下載構建產物

**構建產物:**
- `familydesk-macos-1.4.2` - macOS 可執行文件

---

## ⚠️ 已棄用

### `playground.yml` - 多版本構建 (已棄用)

**狀態:** 🔴 已棄用 - 請勿使用

**問題:**
- ❌ 使用舊的 commit ref
- ❌ 嘗試構建不需要的功能（hwcodec, flutter）
- ❌ 會導致編譯失敗（缺少資源文件）
- ❌ 構建時間長（~30分鐘）
- ❌ 與 FamilyDesk 修改不兼容

**為什麼會失敗:**
```
error: couldn't read `src/../res/mac-tray-dark-x2.png`: No such file or directory
```
- 檢出的舊代碼版本缺少資源文件
- 嘗試構建的功能需要額外依賴

**如果誤觸發了此 workflow:**
1. 立即取消運行
2. 改用 `build-familydesk.yml`

**如果確實需要使用:**
1. 重新觸發
2. 在輸入框中輸入 `YES` 確認
3. 預期會失敗

---

## 📊 Workflow 對比

| 功能 | build-familydesk.yml | playground.yml |
|------|---------------------|----------------|
| **狀態** | ✅ 活躍 | ⚠️ 已棄用 |
| **構建功能** | FamilyDesk 核心 | 多版本 RustDesk |
| **代碼版本** | 當前分支 | 舊 commit ref |
| **構建時間** | ~10分鐘 | ~30分鐘 |
| **成功率** | 高 | 低（會失敗）|
| **產物** | 單個可執行文件 | 4個版本 |
| **依賴** | 最小化 | 複雜（Flutter等）|

---

## 🚀 本地構建（推薦）

如果您不想等待 GitHub Actions，可以本地構建：

```bash
cd /Users/hayden/Downloads/haydendesk

# 使用構建腳本
./build-familydesk.sh

# 或手動構建
cargo build --features family_desk --release
```

**優點:**
- ✅ 更快（~5分鐘）
- ✅ 即時反饋
- ✅ 更好的調試能力
- ✅ 不消耗 GitHub Actions 配額

---

## 📝 創建新的 Workflow

如果需要添加新的構建配置：

1. **複製 `build-familydesk.yml`**
2. **修改名稱和參數**
3. **測試構建**
4. **提交並推送**

**重要:**
- 使用 `workflow_dispatch` 允許手動觸發
- 使用當前分支代碼，不要指定舊的 ref
- 只包含必要的功能標誌

---

## ⚡ 快速參考

### 查看 Workflow 運行狀態
```
https://github.com/YOUR_USERNAME/haydendesk/actions
```

### 手動觸發 Workflow
1. Actions → 選擇 workflow → Run workflow

### 下載構建產物
1. 進入完成的 workflow 運行
2. 滾動到底部 "Artifacts"
3. 點擊下載

### 取消運行中的 Workflow
1. 進入運行中的 workflow
2. 右上角 "Cancel workflow"

---

## 🔧 故障排除

### Workflow 失敗怎麼辦？

1. **檢查使用的是哪個 workflow**
   - ✅ `build-familydesk.yml` - 應該成功
   - ❌ `playground.yml` - 預期失敗

2. **查看錯誤日誌**
   - 點擊失敗的步驟
   - 查看詳細錯誤信息

3. **常見錯誤**
   - 缺少資源文件 → 使用 `build-familydesk.yml`
   - 依賴安裝失敗 → 檢查網絡連接
   - 編譯錯誤 → 檢查代碼是否有語法錯誤

4. **聯繫支持**
   - 查看 `BUILD_TROUBLESHOOTING.md`
   - 創建 GitHub Issue

---

## 📚 相關文檔

- `BUILD_TROUBLESHOOTING.md` - 構建問題排查
- `QUICK_REFERENCE.md` - 快速參考
- `README.md` - 項目主文檔

---

**推薦:** 使用 `build-familydesk.yml` 或本地構建
**避免:** 使用 `playground.yml`（已棄用）

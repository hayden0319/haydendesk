# vcpkg 設置指南

**項目使用 vcpkg Manifest 模式**

---

## 📘 什麼是 Manifest 模式？

FamilyDesk 項目包含 `vcpkg.json` 文件，這意味著使用 **vcpkg manifest 模式**。

**優勢：**
- ✅ 依賴自動管理
- ✅ 版本鎖定（baseline）
- ✅ 可重現構建
- ✅ 不需要手動指定包名

**與傳統模式的區別：**

```bash
# ❌ 傳統模式（已棄用）
vcpkg install libvpx libyuv opus aom

# ✅ Manifest 模式（自動讀取 vcpkg.json）
vcpkg install
```

---

## 🚀 快速設置

### macOS

```bash
# 1. 克隆 vcpkg
git clone https://github.com/microsoft/vcpkg
cd vcpkg

# 2. 檢出指定版本（與 vcpkg.json 中的 baseline 一致）
git checkout 120deac3062162151622ca4860575a33844ba10b

# 3. 初始化 vcpkg
./bootstrap-vcpkg.sh

# 4. 設置環境變量
export VCPKG_ROOT=$PWD

# 5. 返回項目目錄
cd /Users/hayden/Downloads/haydendesk

# 6. 安裝依賴（自動從 vcpkg.json 讀取）
$VCPKG_ROOT/vcpkg install --triplet x64-osx
```

### Linux

```bash
# 1. 克隆 vcpkg
git clone https://github.com/microsoft/vcpkg
cd vcpkg

# 2. 檢出指定版本
git checkout 120deac3062162151622ca4860575a33844ba10b

# 3. 初始化 vcpkg
./bootstrap-vcpkg.sh

# 4. 設置環境變量
export VCPKG_ROOT=$PWD

# 5. 返回項目目錄
cd /path/to/haydendesk

# 6. 安裝依賴
$VCPKG_ROOT/vcpkg install --triplet x64-linux
```

---

## 📋 vcpkg.json 內容

項目的 `vcpkg.json` 包含以下依賴：

```json
{
  "dependencies": [
    { "name": "aom", "host": true },
    { "name": "aom", "host": false },
    { "name": "libjpeg-turbo", "host": true },
    { "name": "libjpeg-turbo", "host": false },
    { "name": "opus", "host": true },
    { "name": "opus", "host": false },
    { "name": "libvpx", "host": true },
    { "name": "libvpx", "host": false },
    { "name": "libyuv", "host": true },
    { "name": "libyuv", "host": false },
    { "name": "ffmpeg", "host": true, "platform": "..." },
    // 更多依賴...
  ],
  "vcpkg-configuration": {
    "default-registry": {
      "kind": "builtin",
      "baseline": "120deac3062162151622ca4860575a33844ba10b"
    },
    "overlay-ports": ["./res/vcpkg"]
  }
}
```

**重要信息：**
- **baseline:** `120deac3062162151622ca4860575a33844ba10b` - 鎖定 vcpkg 版本
- **overlay-ports:** 使用項目本地的 port 覆蓋

---

## ⚠️ 常見錯誤

### 錯誤 1: "In manifest mode, vcpkg install does not support individual package arguments"

**原因：** 嘗試在 manifest 模式下手動指定包名

```bash
# ❌ 錯誤
vcpkg install libvpx libyuv opus aom

# ✅ 正確
vcpkg install
```

**解決：** 不要指定包名，vcpkg 會自動讀取 `vcpkg.json`

### 錯誤 2: "Embedding vcpkg-configuration in a manifest file is an EXPERIMENTAL feature"

**原因：** vcpkg 版本與 baseline 不匹配

**解決：**
```bash
cd vcpkg
git checkout 120deac3062162151622ca4860575a33844ba10b
./bootstrap-vcpkg.sh
```

### 錯誤 3: VCPKG_ROOT 未設置

**錯誤信息：**
```
Could not locate a vcpkg root
```

**解決：**
```bash
export VCPKG_ROOT=/path/to/vcpkg
```

---

## 🔧 Triplet 選擇

### macOS

```bash
# Intel Mac (x86_64)
vcpkg install --triplet x64-osx

# Apple Silicon (ARM64)
vcpkg install --triplet arm64-osx
```

### Linux

```bash
# x86_64
vcpkg install --triplet x64-linux

# ARM64
vcpkg install --triplet arm64-linux
```

### Windows

```bash
# x64
vcpkg install --triplet x64-windows

# x86
vcpkg install --triplet x86-windows
```

---

## 📝 完整設置示例

```bash
#!/bin/bash

# 設置 vcpkg（只需執行一次）
setup_vcpkg() {
    echo "🔧 設置 vcpkg..."

    # 克隆 vcpkg
    if [ ! -d "vcpkg" ]; then
        git clone https://github.com/microsoft/vcpkg
    fi

    cd vcpkg

    # 檢出正確版本
    git checkout 120deac3062162151622ca4860575a33844ba10b

    # 初始化
    ./bootstrap-vcpkg.sh

    # 設置環境變量
    export VCPKG_ROOT=$PWD
    echo "export VCPKG_ROOT=$PWD" >> ~/.bashrc  # 永久保存

    echo "✅ vcpkg 設置完成！"
    echo "VCPKG_ROOT=$VCPKG_ROOT"
}

# 安裝依賴（每次更新 vcpkg.json 後執行）
install_dependencies() {
    echo "📦 安裝 vcpkg 依賴..."

    if [ -z "$VCPKG_ROOT" ]; then
        echo "❌ 錯誤: VCPKG_ROOT 未設置"
        echo "請先運行: export VCPKG_ROOT=/path/to/vcpkg"
        exit 1
    fi

    # 檢測平台
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        ARCH=$(uname -m)
        if [ "$ARCH" = "arm64" ]; then
            TRIPLET="arm64-osx"
        else
            TRIPLET="x64-osx"
        fi
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        TRIPLET="x64-linux"
    else
        echo "⚠️ 未知平台，使用默認 triplet"
        TRIPLET=""
    fi

    # 安裝依賴
    cd /Users/hayden/Downloads/haydendesk
    if [ -n "$TRIPLET" ]; then
        $VCPKG_ROOT/vcpkg install --triplet $TRIPLET
    else
        $VCPKG_ROOT/vcpkg install
    fi

    echo "✅ 依賴安裝完成！"
}

# 執行設置
setup_vcpkg
install_dependencies
```

---

## 🧪 驗證安裝

```bash
# 檢查 VCPKG_ROOT
echo $VCPKG_ROOT
# 應該顯示: /path/to/vcpkg

# 檢查 vcpkg 版本
$VCPKG_ROOT/vcpkg version

# 列出已安裝的包
$VCPKG_ROOT/vcpkg list

# 應該看到:
# aom:x64-osx
# libvpx:x64-osx
# libyuv:x64-osx
# opus:x64-osx
# ...
```

---

## 🔄 更新依賴

如果修改了 `vcpkg.json`：

```bash
# 1. 確保在項目目錄
cd /Users/hayden/Downloads/haydendesk

# 2. 重新安裝
$VCPKG_ROOT/vcpkg install --triplet x64-osx

# 3. 如果需要清理舊依賴
$VCPKG_ROOT/vcpkg remove --outdated
```

---

## 🎯 與構建腳本集成

在 `build-familydesk.sh` 中已經集成了 vcpkg 檢查：

```bash
# 檢查 VCPKG_ROOT
if [ -z "$VCPKG_ROOT" ]; then
    echo "❌ 錯誤: VCPKG_ROOT 未設置"
    echo "請先設置: export VCPKG_ROOT=/path/to/vcpkg"
    exit 1
fi

# 構建
cargo build --features family_desk --release
```

---

## 📚 參考資源

- [vcpkg Manifest Mode 官方文檔](https://learn.microsoft.com/vcpkg/users/manifests)
- [vcpkg.json 參考](https://learn.microsoft.com/vcpkg/reference/vcpkg-json)
- [vcpkg Baseline](https://learn.microsoft.com/vcpkg/users/versioning)

---

## 💡 提示

1. **永久設置 VCPKG_ROOT:**
   ```bash
   echo "export VCPKG_ROOT=/path/to/vcpkg" >> ~/.bashrc
   source ~/.bashrc
   ```

2. **加速安裝（使用二進制緩存）:**
   ```bash
   vcpkg install --binarysource=clear
   ```

3. **僅安裝特定 triplet:**
   ```bash
   vcpkg install --triplet x64-osx --only-downloads
   ```

---

**推薦工作流程:**
1. 設置 vcpkg（一次性）
2. 設置 VCPKG_ROOT 環境變量
3. 運行 `vcpkg install`
4. 運行 `./build-familydesk.sh`

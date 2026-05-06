# Android UI 开发计划

## 现状

当前实现：
- ✅ 核心 Rust 库（Hook 引擎）
- ✅ 命令行工具（hma-rust, symbol-test）
- ❌ Android UI 界面

原版功能：
- ✅ 完整的 Android 应用界面
- ✅ 可视化配置管理
- ✅ 应用列表选择
- ✅ 模板管理
- ✅ 实时日志查看

## 架构设计

```
┌─────────────────────────────────────────┐
│     Android App (Kotlin/Compose)        │
│  ┌───────────────────────────────────┐  │
│  │  UI Layer                          │  │
│  │  - 主界面                          │  │
│  │  - 配置界面                        │  │
│  │  - 应用选择器                      │  │
│  │  - 日志查看器                      │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  ViewModel Layer                   │  │
│  │  - 配置管理                        │  │
│  │  - 状态管理                        │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  JNI Bridge                        │  │
│  │  - Rust FFI 调用                   │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
              ↓ JNI
┌─────────────────────────────────────────┐
│     Rust Core Library                   │
│  - Hook 引擎                            │
│  - 符号解析                             │
│  - PMS Hook                             │
└─────────────────────────────────────────┘
```

## 技术栈

### Android 端
- **语言：** Kotlin
- **UI：** Jetpack Compose
- **架构：** MVVM
- **依赖注入：** Hilt
- **数据存储：** DataStore
- **异步：** Coroutines + Flow

### Rust 端
- **FFI：** JNI 绑定
- **库类型：** cdylib (动态库)
- **接口：** C ABI

## 功能模块

### 1. 主界面
- Hook 状态显示
- 快速开关
- 统计信息
- 设置入口

### 2. 应用管理
- 已安装应用列表
- 搜索和过滤
- 批量选择
- 应用信息显示

### 3. 配置管理
- Scope 配置
- 黑白名单切换
- 模板管理
- 导入/导出配置

### 4. 模板管理
- 预设模板
- 自定义模板
- 模板编辑
- 模板应用

### 5. 日志查看
- 实时日志
- 日志过滤
- 日志导出
- 清除日志

### 6. 设置
- Hook 选项
- 性能设置
- 关于信息
- 更新检查

## 实现步骤

### 阶段 1: JNI 桥接 (1 周)

1. **Rust FFI 接口**
```rust
// lib.rs
#[no_mangle]
pub extern "C" fn hma_init() -> i32 { }

#[no_mangle]
pub extern "C" fn hma_install_hook(config_json: *const c_char) -> i32 { }

#[no_mangle]
pub extern "C" fn hma_uninstall_hook() -> i32 { }

#[no_mangle]
pub extern "C" fn hma_get_status() -> *const c_char { }
```

2. **Kotlin JNI 包装**
```kotlin
object HmaCore {
    external fun init(): Int
    external fun installHook(configJson: String): Int
    external fun uninstallHook(): Int
    external fun getStatus(): String
    
    init {
        System.loadLibrary("hide_my_applist_rust")
    }
}
```

### 阶段 2: Android 项目结构 (3 天)

```
android/
├── app/
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/com/hma/
│   │   │   │   ├── ui/          # UI 组件
│   │   │   │   ├── viewmodel/   # ViewModel
│   │   │   │   ├── data/        # 数据层
│   │   │   │   ├── native/      # JNI 桥接
│   │   │   │   └── MainActivity.kt
│   │   │   ├── res/             # 资源文件
│   │   │   └── AndroidManifest.xml
│   │   └── jniLibs/
│   │       └── arm64-v8a/
│   │           └── libhide_my_applist_rust.so
│   └── build.gradle.kts
├── build.gradle.kts
└── settings.gradle.kts
```

### 阶段 3: 核心 UI (1 周)

1. **主界面**
```kotlin
@Composable
fun MainScreen(viewModel: MainViewModel) {
    Column {
        StatusCard(viewModel.hookStatus)
        QuickActions(viewModel)
        StatisticsCard(viewModel.stats)
    }
}
```

2. **应用列表**
```kotlin
@Composable
fun AppListScreen(viewModel: AppListViewModel) {
    LazyColumn {
        items(viewModel.apps) { app ->
            AppItem(app, onToggle = { viewModel.toggleApp(app) })
        }
    }
}
```

3. **配置界面**
```kotlin
@Composable
fun ConfigScreen(viewModel: ConfigViewModel) {
    Column {
        ScopeSelector(viewModel.scopes)
        ModeSwitch(viewModel.useWhitelist)
        TemplateSelector(viewModel.templates)
    }
}
```

### 阶段 4: 高级功能 (1 周)

1. 模板管理
2. 日志查看
3. 设置界面
4. 导入/导出

### 阶段 5: 优化和测试 (3 天)

1. 性能优化
2. UI 优化
3. 完整测试
4. Bug 修复

## 开发时间估算

| 阶段 | 时间 | 说明 |
|------|------|------|
| JNI 桥接 | 1 周 | Rust FFI + Kotlin 包装 |
| 项目结构 | 3 天 | Android 项目搭建 |
| 核心 UI | 1 周 | 主要界面实现 |
| 高级功能 | 1 周 | 模板、日志等 |
| 优化测试 | 3 天 | 优化和测试 |
| **总计** | **3-4 周** | |

## 快速原型方案

如果需要快速实现，可以采用简化方案：

### 方案 A: WebView + HTML
- 使用 WebView 加载本地 HTML
- JavaScript 调用 Rust 通过 JNI
- 快速开发，但性能较差

### 方案 B: Flutter
- 使用 Flutter 开发 UI
- 通过 FFI 调用 Rust
- 跨平台，开发快速

### 方案 C: 原生 Kotlin + Compose (推荐)
- 性能最好
- 用户体验最佳
- 开发时间适中

## 当前替代方案

在 UI 开发完成前，可以使用：

### 1. 命令行工具
```bash
# 当前可用
hma-rust install config.json
symbol-test version
```

### 2. 配置文件编辑
```bash
# 编辑配置
vim config.json

# 推送到设备
adb push config.json /data/local/tmp/
```

### 3. Shell 脚本
```bash
#!/system/bin/sh
# hma-manager.sh

case "$1" in
    start)
        /data/local/tmp/hma-rust install /data/local/tmp/config.json
        ;;
    stop)
        /data/local/tmp/hma-rust uninstall
        ;;
    status)
        /data/local/tmp/hma-rust test
        ;;
esac
```

### 4. Termux 应用
- 在 Termux 中运行命令行工具
- 提供基本的交互界面

## 下一步行动

### 立即可做
1. ✅ 完成核心 Rust 库（已完成）
2. ✅ 实现命令行工具（已完成）
3. ⏳ 编写 JNI 接口
4. ⏳ 创建 Android 项目

### 短期计划 (1-2 周)
1. 实现 JNI 桥接
2. 创建基础 Android 项目
3. 实现主界面

### 中期计划 (1 个月)
1. 完整 UI 实现
2. 功能测试
3. 发布 Beta 版本

## 参考资料

- [原版 Hide-My-Applist UI](https://github.com/Dr-TSNG/Hide-My-Applist)
- [Jetpack Compose 文档](https://developer.android.com/jetpack/compose)
- [Rust JNI 绑定](https://github.com/jni-rs/jni-rs)
- [Android NDK 文档](https://developer.android.com/ndk)

## 总结

**当前状态：**
- ✅ 核心功能完成（Rust 库 + CLI）
- ❌ Android UI 未实现

**优先级：**
1. 高：JNI 桥接
2. 高：基础 UI
3. 中：高级功能
4. 低：优化和美化

**预计时间：** 3-4 周完整实现

**临时方案：** 使用命令行工具 + 配置文件

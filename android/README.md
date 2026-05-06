# Android UI 使用说明

## 项目结构

```
android/
├── app/
│   ├── src/main/
│   │   ├── java/com/hma/
│   │   │   ├── MainActivity.kt          # 主Activity
│   │   │   ├── ui/
│   │   │   │   ├── MainScreen.kt        # 主界面
│   │   │   │   └── AppListScreen.kt     # 应用列表
│   │   │   ├── viewmodel/
│   │   │   │   └── MainViewModel.kt     # ViewModel
│   │   │   ├── data/
│   │   │   │   └── Models.kt            # 数据模型
│   │   │   └── native/
│   │   │       └── HmaCore.kt           # JNI 桥接
│   │   ├── jniLibs/arm64-v8a/
│   │   │   └── libhide_my_applist_rust.so
│   │   └── AndroidManifest.xml
│   └── build.gradle.kts
├── build.gradle.kts
├── settings.gradle.kts
└── build-android.sh                     # 构建脚本
```

## 功能特性

### 已实现
- ✅ 主界面（Hook 状态、快速开关）
- ✅ 应用列表（搜索、选择）
- ✅ JNI 桥接（Rust ↔ Kotlin）
- ✅ Material 3 设计
- ✅ Jetpack Compose UI

### 待实现
- ⏳ 配置管理界面
- ⏳ 模板管理
- ⏳ 日志查看
- ⏳ 设置界面

## 构建步骤

### 前置要求

1. **Android Studio**
   - 版本：Hedgehog (2023.1.1) 或更高
   - SDK: API 34

2. **Android NDK**
   ```bash
   export ANDROID_NDK_HOME=/path/to/ndk
   ```

3. **Rust 工具链**
   ```bash
   rustup target add aarch64-linux-android
   ```

### 构建命令

#### 方法 1: 使用构建脚本（推荐）
```bash
cd android
./build-android.sh
```

#### 方法 2: 手动构建
```bash
# 1. 构建 Rust 库
cargo build --release --target aarch64-linux-android --features android

# 2. 复制库文件
cp target/aarch64-linux-android/release/libhide_my_applist_rust.so \
   android/app/src/main/jniLibs/arm64-v8a/

# 3. 构建 APK
cd android
./gradlew assembleRelease
```

#### 方法 3: Android Studio
1. 打开 `android/` 目录
2. 等待 Gradle 同步
3. Build → Build Bundle(s) / APK(s) → Build APK(s)

## 安装和使用

### 安装
```bash
adb install android/app/build/outputs/apk/release/app-release.apk
```

### 使用流程

1. **打开应用**
   - 首次打开会检查 wxshadow 状态

2. **配置应用**
   - 点击"应用管理"
   - 搜索并选择要隐藏的应用
   - 返回主界面

3. **启用 Hook**
   - 在主界面打开 Hook 开关
   - 等待状态变为"已激活"

4. **验证效果**
   - 打开目标应用
   - 检查应用列表是否被隐藏

## 界面说明

### 主界面
```
┌─────────────────────────────┐
│  Hide My Applist            │
├─────────────────────────────┤
│  ┌───────────────────────┐  │
│  │ Hook 状态              │  │
│  │ 已激活          [ON]  │  │
│  │ 过滤次数: 123         │  │
│  └───────────────────────┘  │
│                             │
│  [    应用管理    ]         │
│  [    配置管理    ]         │
│  [    设置        ]         │
└─────────────────────────────┘
```

### 应用列表
```
┌─────────────────────────────┐
│  [搜索...]                  │
├─────────────────────────────┤
│  📱 Chrome              [✓] │
│  📱 设置                [ ] │
│  📱 Magisk              [✓] │
│  ...                        │
└─────────────────────────────┘
```

## 技术栈

- **UI 框架：** Jetpack Compose
- **架构：** MVVM
- **语言：** Kotlin
- **最低 SDK：** 28 (Android 9)
- **目标 SDK：** 34 (Android 14)

## 依赖库

```kotlin
// Compose
androidx.compose.ui:ui
androidx.compose.material3:material3
androidx.navigation:navigation-compose

// ViewModel
androidx.lifecycle:lifecycle-viewmodel-compose

// 其他
androidx.core:core-ktx
androidx.activity:activity-compose
```

## 开发指南

### 添加新界面

1. 在 `ui/` 创建新的 Composable
2. 在 `MainActivity.kt` 添加路由
3. 在 `ViewModel` 添加状态管理

### 调用 Rust 函数

```kotlin
// 在 Kotlin 中
val result = HmaCore.installHook(configJson)

// 对应的 Rust 函数
#[no_mangle]
pub extern "system" fn Java_com_hma_native_HmaCore_installHook(
    env: JNIEnv,
    _class: JClass,
    config_json: JString
) -> jint {
    // 实现
}
```

### 调试

```bash
# 查看日志
adb logcat | grep HMA

# 查看 JNI 日志
adb logcat | grep "art:"
```

## 常见问题

### Q1: 找不到 libhide_my_applist_rust.so

**解决：**
```bash
# 确保库文件存在
ls android/app/src/main/jniLibs/arm64-v8a/

# 重新构建
./build-android.sh
```

### Q2: JNI 调用失败

**解决：**
```bash
# 检查符号
nm -D libhide_my_applist_rust.so | grep Java_com_hma

# 查看日志
adb logcat | grep "UnsatisfiedLinkError"
```

### Q3: 编译错误

**解决：**
```bash
# 清理缓存
cd android
./gradlew clean

# 重新构建
./gradlew assembleRelease
```

## 下一步

1. ⏳ 完善配置管理界面
2. ⏳ 添加模板管理
3. ⏳ 实现日志查看
4. ⏳ 添加设置选项
5. ⏳ 优化 UI/UX
6. ⏳ 添加单元测试

## 参考资料

- [Jetpack Compose 文档](https://developer.android.com/jetpack/compose)
- [JNI 规范](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/)
- [Rust JNI 绑定](https://github.com/jni-rs/jni-rs)

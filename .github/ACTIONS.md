# GitHub Actions 配置说明

## 工作流文件

### 1. build.yml - 构建工作流
**触发条件：**
- Push 到 main/master 分支
- Pull Request
- 手动触发

**任务：**
1. **build-rust** - 构建 Rust 库
   - 设置 Rust 工具链
   - 安装 Android NDK
   - 编译 aarch64-linux-android 目标
   - 上传 .so 文件

2. **build-android** - 构建 Android APK
   - 设置 JDK 17
   - 下载 Rust 库
   - 编译 APK
   - 上传 APK

### 2. release.yml - 发布工作流
**触发条件：**
- Push tag (v*)
- 手动触发

**任务：**
- 构建 Rust 库和 Android APK
- 创建 GitHub Release
- 上传构建产物
- 生成发布说明

### 3. test.yml - 测试工作流
**触发条件：**
- Push 到 main/master
- Pull Request

**任务：**
- Rust 测试（cargo test）
- Rust Clippy 检查
- Rust 格式检查
- Android 单元测试

## 使用方法

### 首次设置

1. **创建 GitHub 仓库**
```bash
cd hide-my-applist-rust
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR_USERNAME/hide-my-applist-rust.git
git push -u origin main
```

2. **配置 Secrets（可选）**
如果需要签名 APK，在 GitHub 仓库设置中添加：
- `KEYSTORE_FILE` - Base64 编码的 keystore
- `KEYSTORE_PASSWORD` - Keystore 密码
- `KEY_ALIAS` - Key 别名
- `KEY_PASSWORD` - Key 密码

### 触发构建

**自动触发：**
```bash
git push origin main
```

**手动触发：**
1. 进入 GitHub 仓库
2. 点击 "Actions" 标签
3. 选择工作流
4. 点击 "Run workflow"

### 创建发布

```bash
# 创建 tag
git tag v0.2.0
git push origin v0.2.0

# 自动触发 release 工作流
# 构建完成后会创建 GitHub Release
```

## 构建产物

### 构建工作流
- `rust-library` - libhide_my_applist_rust.so
- `app-release` - app-release.apk

### 发布工作流
- GitHub Release 包含：
  - app-release.apk
  - libhide_my_applist_rust.so
  - 自动生成的发布说明

## 缓存策略

### Cargo 缓存
- `~/.cargo/registry`
- `~/.cargo/git`
- `target/`

### Gradle 缓存
- `~/.gradle/caches`
- `~/.gradle/wrapper`

缓存 key 基于：
- Cargo.lock
- *.gradle*
- gradle-wrapper.properties

## 优化建议

### 1. 并行构建
当前配置已经使用了并行任务：
- Rust 和 Android 分开构建
- 使用 artifact 传递

### 2. 缓存优化
- Cargo 缓存可以节省 5-10 分钟
- Gradle 缓存可以节省 2-5 分钟

### 3. 构建时间
预计构建时间：
- Rust 库：5-8 分钟
- Android APK：3-5 分钟
- 总计：8-13 分钟

## 故障排除

### NDK 问题
如果 NDK 设置失败：
```yaml
- name: Setup Android NDK
  uses: nttld/setup-ndk@v1
  with:
    ndk-version: r26b  # 尝试其他版本
```

### Rust 目标问题
如果目标安装失败：
```bash
rustup target add aarch64-linux-android
```

### Gradle 问题
如果 Gradle 构建失败：
```bash
cd android
./gradlew clean
./gradlew assembleRelease --stacktrace
```

## 本地测试

在推送前本地测试：

```bash
# 测试 Rust 构建
cargo build --release --target aarch64-linux-android --features android

# 测试 Android 构建
cd android
./gradlew assembleRelease

# 运行测试
cargo test
cd android && ./gradlew test
```

## 高级配置

### 添加签名

在 `build.yml` 中添加：

```yaml
- name: Sign APK
  run: |
    echo "${{ secrets.KEYSTORE_FILE }}" | base64 -d > keystore.jks
    jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 \
      -keystore keystore.jks \
      -storepass "${{ secrets.KEYSTORE_PASSWORD }}" \
      -keypass "${{ secrets.KEY_PASSWORD }}" \
      android/app/build/outputs/apk/release/app-release-unsigned.apk \
      "${{ secrets.KEY_ALIAS }}"
    zipalign -v 4 \
      android/app/build/outputs/apk/release/app-release-unsigned.apk \
      android/app/build/outputs/apk/release/app-release.apk
```

### 多架构支持

添加其他架构：

```yaml
strategy:
  matrix:
    target: [aarch64-linux-android, armv7-linux-androideabi]
```

## 监控

### 查看构建状态
- GitHub Actions 标签页
- 提交旁边的状态图标
- README 中的徽章

### 添加状态徽章

在 README.md 中添加：

```markdown
![Build](https://github.com/YOUR_USERNAME/hide-my-applist-rust/workflows/Build/badge.svg)
![Test](https://github.com/YOUR_USERNAME/hide-my-applist-rust/workflows/Test/badge.svg)
```

## 参考资料

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Rust Actions](https://github.com/actions-rs)
- [Android Actions](https://github.com/android-actions)

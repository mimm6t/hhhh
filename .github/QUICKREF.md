# GitHub Actions 快速参考

## 📁 文件清单

```
.github/
├── workflows/
│   ├── build.yml      # 构建工作流
│   ├── release.yml    # 发布工作流
│   └── test.yml       # 测试工作流
└── ACTIONS.md         # 详细说明

.cargo/
└── config.toml        # Cargo 配置

.gitignore             # Git 忽略
CHANGELOG.md           # 更新日志
```

## 🚀 快速开始

### 1. 推送到 GitHub
```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR_USERNAME/hide-my-applist-rust.git
git push -u origin main
```

### 2. 触发构建
```bash
git push origin main
```

### 3. 创建发布
```bash
git tag v0.2.0
git push origin v0.2.0
```

## 📊 工作流

| 工作流 | 触发 | 产物 |
|--------|------|------|
| build.yml | Push/PR | APK + .so |
| release.yml | Tag | GitHub Release |
| test.yml | Push/PR | 测试报告 |

## ⏱️ 构建时间

- Rust 库：5-8 分钟
- Android APK：3-5 分钟
- 测试：2-3 分钟
- **总计：10-16 分钟**

## 🎯 缓存

- Cargo：~/.cargo + target/
- Gradle：~/.gradle
- **节省：7-15 分钟**

## 📦 下载产物

### 从 Actions
Actions → 选择运行 → Artifacts

### 从 Release
Releases → 最新版本 → Assets

## 🔧 本地测试

```bash
# Rust
cargo test
cargo clippy

# Android
cd android && ./gradlew test
```

## 📝 更新 README

替换 YOUR_USERNAME：
```markdown
![Build](https://github.com/YOUR_USERNAME/hide-my-applist-rust/workflows/Build/badge.svg)
```

## 🐛 故障排除

### NDK 问题
修改 `ndk-version: r26b`

### Gradle 问题
```bash
./gradlew clean
./gradlew assembleRelease --stacktrace
```

## 📚 文档

- `.github/ACTIONS.md` - 完整说明
- `GITHUB_ACTIONS_REPORT.md` - 配置报告
- `CHANGELOG.md` - 版本历史

## ✅ 检查清单

- [ ] 推送到 GitHub
- [ ] 验证构建成功
- [ ] 下载测试 APK
- [ ] 创建首个 Release
- [ ] 更新 README 徽章

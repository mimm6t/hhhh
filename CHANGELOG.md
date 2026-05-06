# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-05-06

### Added
- 完整的 Rust 核心库
  - wxshadow FFI 封装
  - ELF 解析器
  - 符号解析系统
  - Android 9-15 版本适配
  - 实际 PMS Hook 实现
  
- 完整的 Android UI
  - 主界面（Hook 状态、快速开关）
  - 应用管理（搜索、选择）
  - 配置管理（Scope、黑白名单）
  - 模板管理（预设、自定义）
  - 日志查看（实时、过滤）
  - 设置界面（导入/导出、更新）
  
- 文件操作
  - 配置保存/加载
  - 配置导入/导出
  - 自动备份
  
- 网络功能
  - 检查更新
  - 下载更新
  
- 数据持久化
  - DataStore 集成
  - 响应式数据流

### Changed
- 从 Xposed 框架迁移到 wxshadow 内核模块
- 使用 Jetpack Compose 替代 XML 布局
- 使用 Rust 实现核心库

### Technical
- 内核级别 Hook
- W^X Shadow 技术
- 自动符号解析
- 多版本自动适配

## [0.1.0] - 2026-05-06

### Added
- 初始项目结构
- 基础 Rust 库
- 命令行工具

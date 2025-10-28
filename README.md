# Mingbu

> ⚠️ 本项目正处于快速开发阶段，API可能会频繁变动。

Mingbu (命卜) 是一个用于中国玄学计算和占卜的Rust库。这个名字来源于中国传统五术中的两个分支：命（命理推算）和卜（占卜）。

## 功能特性

- 中国玄学基本概念
  - 阴阳
  - 五行
  - 八卦
  - 天干地支
- 元素之间的关系计算
- 可扩展的特征系统，支持自定义实现

## 编译

要编译此项目，您需要安装Rust工具链以及一些平台特定的依赖项。以下是不同平台上的编译指南：

### 通用要求

1. 安装 Rust 工具链: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Linux

```bash
# 安装构建依赖
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libclang-dev

# 克隆并构建项目
git clone <repository-url>
cd mingbu
cargo build --release
```

### macOS

```bash
# 克隆并构建项目
git clone <repository-url>
cd mingbu
cargo build --release
```

### Windows

Windows通常不需要额外的系统依赖来编译Rust/C项目，但您需要确保已安装Visual Studio C++构建工具。

```cmd
REM 克隆并构建项目
git clone <repository-url>
cd mingbu
cargo build --release
```

## 使用此库

在`Cargo.toml`中添加以下依赖：

```toml
[dependencies]
mingbu = "0.1.0"
```


## 特别鸣谢

本项目使用了 [Swiss Ephemeris](https://www.astro.com/swisseph/) 库作为天文学计算的基础。Swiss Ephemeris 是一个广泛使用的高精度天体历计算库，最初由Astrodienst开发。


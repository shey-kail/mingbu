# Mingbu

> ⚠️ 本项目正处于快速开发阶段，API可能会频繁变动。

Mingbu (命卜) 是一个用于中国玄学计算和占卜的Rust库。这个名字来源于中国传统五术中的两个分支：命（命理推算）和卜（占卜）。

[![Build Status](https://github.com/shey-kail/mingbu/workflows/Cross-platform%20Build/badge.svg)](https://github.com/shey-kail/mingbu/actions)
[![License](https://img.shields.io/badge/license-AGPL--3.0-orange)](https://github.com/shey-kail/mingbu/blob/master/LICENSE)
[![Platforms](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20Android%20%7C%20iOS-blue)](https://github.com/shey-kail/mingbu)
[![Rust](https://img.shields.io/badge/rust-1.65%2B-orange.svg)](https://www.rust-lang.org)

## 项目状态

### 已完成
- ✅ Swiss Ephemeris库集成
- ✅ 多平台编译支持（Linux、macOS、Windows、Android、iOS）

### 待办事项
- ☐ 仿照Moria项目，完成八字和七政四余功能实现

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

### 移动端编译要求

对于Android和iOS编译，您需要额外的工具链和依赖项。请参考相应平台的详细说明。

### 通用要求

1. 安装 Rust 工具链: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Linux

```bash
# 安装构建依赖
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libclang-dev

# 克隆并构建项目
git clone https://github.com/shey-kail/mingbu
cd mingbu
cargo build --release
```

### macOS

```bash
# 克隆并构建项目
git clone https://github.com/shey-kail/mingbu
cd mingbu
cargo build --release
```

### Windows

Windows通常不需要额外的系统依赖来编译Rust/C项目，但您需要确保已安装Visual Studio C++构建工具。

```powershell
# 克隆并构建项目
git clone https://github.com/shey-kail/mingbu
cd mingbu
cargo build --release
```

### Android

要为Android构建，您需要安装Android NDK和相应的Rust目标：

```bash
# 安装Android目标
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# 构建Android库
cargo build --target aarch64-linux-android --release
```

### iOS

要为iOS构建，您需要安装相应的Rust目标：

```bash
# 安装iOS目标
rustup target add aarch64-apple-ios x86_64-apple-ios

# 构建iOS库
cargo build --target aarch64-apple-ios --release
```

## 使用此库

在`Cargo.toml`中添加以下依赖：

```toml
[dependencies]
mingbu = "0.1.0"
```


## 特别鸣谢

本项目使用了 [Swiss Ephemeris](https://www.astro.com/swisseph/) 库作为天文学计算的基础。Swiss Ephemeris 是一个广泛使用的高精度天体历计算库，最初由Astrodienst开发。

本项目在七政四余的实现上受到了 [Moria](https://sites.google.com/site/athomeprojects) 项目的启发。


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

## 安装

在`Cargo.toml`中添加以下依赖：

```toml
[dependencies]
mingbu = "0.1.0"
```

## 使用示例

```rust
use mingbu::{WuXing, YinYang, EarthlyBranch, HeavenlyStem, Trigram};

// 创建和使用五行
let wood = WuXing::Wood;
let fire = WuXing::Fire;

// 创建和使用八卦
let qian = Trigram::Qian;
let kun = Trigram::Kun;

// 使用天干地支
let jia = HeavenlyStem::Jia;
let zi = EarthlyBranch::Zi;
```

## 许可证

本项目采用MIT许可证 - 详见[LICENSE](LICENSE)文件。
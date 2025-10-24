# Mingbu

> ⚠️ This project is under rapid development and the API may change frequently.

Mingbu (命卜) is a Rust library for Chinese metaphysics calculations and divination. The name comes from two of the Five Arts (五术) in Chinese culture: Ming (命, destiny calculation) and Bu (卜, divination).

## Features

- Basic concepts in Chinese metaphysics
  - Yin-Yang (阴阳)
  - Five Elements (五行)
  - Eight Trigrams (八卦)
  - Heavenly Stems and Earthly Branches (天干地支)
- Relationship calculations between elements
- Extensible trait system for custom implementations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mingbu = "0.1.0"
```

## Usage

```rust
use mingbu::{WuXing, YinYang, EarthlyBranch, HeavenlyStem, Trigram};

// Create and use Five Elements
let wood = WuXing::Wood;
let fire = WuXing::Fire;

// Create and use Trigrams
let qian = Trigram::Qian;
let kun = Trigram::Kun;

// Work with Heavenly Stems and Earthly Branches
let jia = HeavenlyStem::Jia;
let zi = EarthlyBranch::Zi;
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

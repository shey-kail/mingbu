//! 六十甲子模块，定义干支组合的基本类型和特征

use super::{HeavenlyStem, EarthlyBranch, Sound};

/// 六十甲子结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SexagesimalCycle {
    /// 天干
    stem: HeavenlyStem,
    /// 地支
    branch: EarthlyBranch,
    /// 纳音
    sound: Sound,
}

impl SexagesimalCycle {
    /// 创建一个新的六十甲子实例
    pub const fn new(stem: HeavenlyStem, branch: EarthlyBranch, sound: Sound) -> Self {
        Self { stem, branch, sound }
    }

    /// 获取天干
    pub fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// 获取地支
    pub fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// 获取纳音
    pub fn sound(&self) -> Sound {
        self.sound
    }
}

impl std::fmt::Display for SexagesimalCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?} {}", self.stem, self.branch, self.sound)
    }
}
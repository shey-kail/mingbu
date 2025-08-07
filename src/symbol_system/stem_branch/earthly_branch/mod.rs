//! 地支相关类型的模块

pub mod relationship;
pub mod hidden_stem;
mod data;
mod tests;

use crate::symbol_system::{YinYang, WuXing};
use crate::traits::{Index, ChineseName, Iter};
use crate::traits::yinyang_wuxing::{WuXingTrait, YinYangTrait};
/// 地支枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarthlyBranch {
    /// 子
    Zi,
    /// 丑
    Chou,
    /// 寅
    Yin,
    /// 卯
    Mao,
    /// 辰
    Chen,
    /// 巳
    Si,
    /// 午
    Wu,
    /// 未
    Wei,
    /// 申
    Shen,
    /// 酉
    You,
    /// 戌
    Xu,
    /// 亥
    Hai,
}

impl YinYangTrait for EarthlyBranch {
    fn yinyang(&self) -> YinYang {
        match self {
            EarthlyBranch::Zi | EarthlyBranch::Wu | EarthlyBranch::Chen | 
            EarthlyBranch::You | EarthlyBranch::Shen | EarthlyBranch::Yin => YinYang::Yang,
            EarthlyBranch::Chou | EarthlyBranch::Wei | EarthlyBranch::Si | 
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Xu => YinYang::Yin,
        }
    }
}

impl WuXingTrait for EarthlyBranch {
    fn wuxing(&self) -> WuXing {
        match self {
            EarthlyBranch::Yin | EarthlyBranch::Mao => WuXing::Wood,
            EarthlyBranch::Si | EarthlyBranch::Wu => WuXing::Fire,
            EarthlyBranch::Chen | EarthlyBranch::Xu | 
            EarthlyBranch::Chou | EarthlyBranch::Wei => WuXing::Earth,
            EarthlyBranch::Shen | EarthlyBranch::You => WuXing::Metal,
            EarthlyBranch::Hai | EarthlyBranch::Zi => WuXing::Water,
        }
    }
}

impl ChineseName for EarthlyBranch {
    fn chinese_name(&self) -> &'static str {
        match self {
            EarthlyBranch::Zi => "子",
            EarthlyBranch::Chou => "丑",
            EarthlyBranch::Yin => "寅",
            EarthlyBranch::Mao => "卯",
            EarthlyBranch::Chen => "辰",
            EarthlyBranch::Si => "巳",
            EarthlyBranch::Wu => "午",
            EarthlyBranch::Wei => "未",
            EarthlyBranch::Shen => "申",
            EarthlyBranch::You => "酉",
            EarthlyBranch::Xu => "戌",
            EarthlyBranch::Hai => "亥",
        }
    }
}

impl Index for EarthlyBranch {
    fn from_index(index: usize) -> Self {
        match (index - 1) % 12 {
            0 => EarthlyBranch::Zi,
            1 => EarthlyBranch::Chou,
            2 => EarthlyBranch::Yin,
            3 => EarthlyBranch::Mao,
            4 => EarthlyBranch::Chen,
            5 => EarthlyBranch::Si,
            6 => EarthlyBranch::Wu,
            7 => EarthlyBranch::Wei,
            8 => EarthlyBranch::Shen,
            9 => EarthlyBranch::You,
            10 => EarthlyBranch::Xu,
            11 => EarthlyBranch::Hai,
            _ => unreachable!(),
        }
    }
    fn index(&self) -> usize {
        match self {
            EarthlyBranch::Zi => 1,
            EarthlyBranch::Chou => 2,
            EarthlyBranch::Yin => 3,
            EarthlyBranch::Mao => 4,
            EarthlyBranch::Chen => 5,
            EarthlyBranch::Si => 6,
            EarthlyBranch::Wu => 7,
            EarthlyBranch::Wei => 8,
            EarthlyBranch::Shen => 9,
            EarthlyBranch::You => 10,
            EarthlyBranch::Xu => 11,
            EarthlyBranch::Hai => 12,
        }
    }
}

impl Iter for EarthlyBranch {
    type Item = EarthlyBranch;
    fn next(&self) -> Self::Item {
        Self::from_index(self.index() + 1)
    }
    fn prev(&self) -> Self::Item {
        Self::from_index(self.index() + 11)
    }
}
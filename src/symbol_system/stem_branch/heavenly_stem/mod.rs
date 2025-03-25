//! 天干相关类型的模块

pub mod relationship;
mod tests;

use crate::basic::{YinYang, WuXing};
use crate::traits::{ChineseName, Index, Iter};
use crate::traits::yinyang_wuxing::{WuXingTrait, YinYangTrait};

/// 天干枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeavenlyStem {
    /// 甲
    Jia,
    /// 乙
    Yi,
    /// 丙
    Bing,
    /// 丁
    Ding,
    /// 戊
    Wu,
    /// 己
    Ji,
    /// 庚
    Geng,
    /// 辛
    Xin,
    /// 壬
    Ren,
    /// 癸
    Gui,
}

impl YinYangTrait for HeavenlyStem {
    fn yinyang(&self) -> YinYang {
        match self {
            HeavenlyStem::Jia | HeavenlyStem::Bing | HeavenlyStem::Wu | 
            HeavenlyStem::Geng | HeavenlyStem::Ren => YinYang::Yang,
            HeavenlyStem::Yi | HeavenlyStem::Ding | HeavenlyStem::Ji | 
            HeavenlyStem::Xin | HeavenlyStem::Gui => YinYang::Yin,
        }
    }
}

impl WuXingTrait for HeavenlyStem {
    fn wuxing(&self) -> WuXing {
        match self {
            HeavenlyStem::Jia | HeavenlyStem::Yi => WuXing::Wood,
            HeavenlyStem::Bing | HeavenlyStem::Ding => WuXing::Fire,
            HeavenlyStem::Wu | HeavenlyStem::Ji => WuXing::Earth,
            HeavenlyStem::Geng | HeavenlyStem::Xin => WuXing::Metal,
            HeavenlyStem::Ren | HeavenlyStem::Gui => WuXing::Water,
        }
    } 
}

impl ChineseName for HeavenlyStem {
    fn chinese_name(&self) -> &'static str {
        match self {
            HeavenlyStem::Jia => "甲",
            HeavenlyStem::Yi => "乙",
            HeavenlyStem::Bing => "丙",
            HeavenlyStem::Ding => "丁",
            HeavenlyStem::Wu => "戊",
            HeavenlyStem::Ji => "己",
            HeavenlyStem::Geng => "庚",
            HeavenlyStem::Xin => "辛",
            HeavenlyStem::Ren => "壬",
            HeavenlyStem::Gui => "癸",
        }
    }
}

impl Index for HeavenlyStem {
    fn from_index(index: usize) -> Self {
        match (index - 1) % 10 {
            0 => HeavenlyStem::Jia,
            1 => HeavenlyStem::Yi,
            2 => HeavenlyStem::Bing,
            3 => HeavenlyStem::Ding,
            4 => HeavenlyStem::Wu,
            5 => HeavenlyStem::Ji,
            6 => HeavenlyStem::Geng,
            7 => HeavenlyStem::Xin,
            8 => HeavenlyStem::Ren,
            9 => HeavenlyStem::Gui,
            _ => unreachable!(),
        }
    }

    fn index(&self) -> usize {
        match self {
            HeavenlyStem::Jia => 1,
            HeavenlyStem::Yi => 2,
            HeavenlyStem::Bing => 3,
            HeavenlyStem::Ding => 4,
            HeavenlyStem::Wu => 5,
            HeavenlyStem::Ji => 6,
            HeavenlyStem::Geng => 7,
            HeavenlyStem::Xin => 8,
            HeavenlyStem::Ren => 9,
            HeavenlyStem::Gui => 10,
        }
    }
}

impl Iter for HeavenlyStem {
    type Item = Self;

    fn next(&self) -> Self::Item {
        Self::from_index(self.index() + 1)
    }

    fn prev(&self) -> Self::Item {
        Self::from_index(self.index() + 9)
    }
}

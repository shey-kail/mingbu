//! 天干相关类型的模块

mod relationship;

use super::Sound;

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

pub use relationship::HeavenlyStemRelationship;
//! 五行关系模块，定义五行之间的生克关系

use super::WuXing;
use crate::traits::Relationship;

/// 五行关系枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WuXingRelation {
    /// 生
    Generating,
    /// 克
    Overcoming,
    /// 同
    Same,
    /// 被生
    BeingGenerated,
    /// 被克
    BeingOvercome,
}

impl std::fmt::Display for WuXingRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WuXingRelation::Generating => write!(f, "生"),
            WuXingRelation::Overcoming => write!(f, "克"),
            WuXingRelation::Same => write!(f, "同"),
            WuXingRelation::BeingGenerated => write!(f, "被生"),
            WuXingRelation::BeingOvercome => write!(f, "被克"),
        }
    }
}

impl Relationship for WuXing {
    type Item = WuXingRelation;

    fn relationship_with(&self, other: &Self) -> Self::Item {
        match (self, other) {
            // 木生火
            (WuXing::Wood, WuXing::Fire) => WuXingRelation::Generating,
            (WuXing::Fire, WuXing::Wood) => WuXingRelation::BeingGenerated,
            // 火生土
            (WuXing::Fire, WuXing::Earth) => WuXingRelation::Generating,
            (WuXing::Earth, WuXing::Fire) => WuXingRelation::BeingGenerated,
            // 土生金
            (WuXing::Earth, WuXing::Metal) => WuXingRelation::Generating,
            (WuXing::Metal, WuXing::Earth) => WuXingRelation::BeingGenerated,
            // 金生水
            (WuXing::Metal, WuXing::Water) => WuXingRelation::Generating,
            (WuXing::Water, WuXing::Metal) => WuXingRelation::BeingGenerated,
            // 水生木
            (WuXing::Water, WuXing::Wood) => WuXingRelation::Generating,
            (WuXing::Wood, WuXing::Water) => WuXingRelation::BeingGenerated,
            // 木克土
            (WuXing::Wood, WuXing::Earth) => WuXingRelation::Overcoming,
            (WuXing::Earth, WuXing::Wood) => WuXingRelation::BeingOvercome,
            // 土克水
            (WuXing::Earth, WuXing::Water) => WuXingRelation::Overcoming,
            (WuXing::Water, WuXing::Earth) => WuXingRelation::BeingOvercome,
            // 水克火
            (WuXing::Water, WuXing::Fire) => WuXingRelation::Overcoming,
            (WuXing::Fire, WuXing::Water) => WuXingRelation::BeingOvercome,
            // 火克金
            (WuXing::Fire, WuXing::Metal) => WuXingRelation::Overcoming,
            (WuXing::Metal, WuXing::Fire) => WuXingRelation::BeingOvercome,
            // 金克木
            (WuXing::Metal, WuXing::Wood) => WuXingRelation::Overcoming,
            (WuXing::Wood, WuXing::Metal) => WuXingRelation::BeingOvercome,

            // 其他情况就是相同
            _ => WuXingRelation::Same,
        }
    }

    fn from_relationship(&self, relationship: Self::Item) -> Self {
        match relationship {
            WuXingRelation::Generating => match self {
                WuXing::Wood => WuXing::Fire,
                WuXing::Fire => WuXing::Earth,
                WuXing::Earth => WuXing::Metal,
                WuXing::Metal => WuXing::Water,
                WuXing::Water => WuXing::Wood,
            },
            WuXingRelation::BeingGenerated => match self {
                WuXing::Wood => WuXing::Water,
                WuXing::Fire => WuXing::Wood,
                WuXing::Earth => WuXing::Fire,
                WuXing::Metal => WuXing::Earth,
                WuXing::Water => WuXing::Metal,
            },
            WuXingRelation::Overcoming => match self {
                WuXing::Wood => WuXing::Earth,
                WuXing::Fire => WuXing::Metal,
                WuXing::Earth => WuXing::Water,
                WuXing::Metal => WuXing::Wood,
                WuXing::Water => WuXing::Fire,
            },
            WuXingRelation::BeingOvercome => match self {
                WuXing::Wood => WuXing::Metal,
                WuXing::Fire => WuXing::Water,
                WuXing::Earth => WuXing::Wood,
                WuXing::Metal => WuXing::Fire,
                WuXing::Water => WuXing::Earth,
            },
            WuXingRelation::Same => match self {
                WuXing::Wood => WuXing::Metal,
                WuXing::Fire => WuXing::Water,
                WuXing::Earth => WuXing::Wood,
                WuXing::Metal => WuXing::Fire,
                WuXing::Water => WuXing::Earth,
            }
        }
    }
}
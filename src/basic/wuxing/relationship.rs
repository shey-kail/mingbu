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

    fn relationship_with(&self, other: &Self) -> Vec<Self::Item> {
        match (self, other) {
            // 木生火
            (WuXing::Wood, WuXing::Fire) => vec![WuXingRelation::Generating],
            (WuXing::Fire, WuXing::Wood) => vec![WuXingRelation::BeingGenerated],
            // 火生土
            (WuXing::Fire, WuXing::Earth) => vec![WuXingRelation::Generating],
            (WuXing::Earth, WuXing::Fire) => vec![WuXingRelation::BeingGenerated],
            // 土生金
            (WuXing::Earth, WuXing::Metal) => vec![WuXingRelation::Generating],
            (WuXing::Metal, WuXing::Earth) => vec![WuXingRelation::BeingGenerated],
            // 金生水
            (WuXing::Metal, WuXing::Water) => vec![WuXingRelation::Generating],
            (WuXing::Water, WuXing::Metal) => vec![WuXingRelation::BeingGenerated],
            // 水生木
            (WuXing::Water, WuXing::Wood) => vec![WuXingRelation::Generating],
            (WuXing::Wood, WuXing::Water) => vec![WuXingRelation::BeingGenerated],
            // 木克土
            (WuXing::Wood, WuXing::Earth) => vec![WuXingRelation::Overcoming],
            (WuXing::Earth, WuXing::Wood) => vec![WuXingRelation::BeingOvercome],
            // 土克水
            (WuXing::Earth, WuXing::Water) => vec![WuXingRelation::Overcoming],
            (WuXing::Water, WuXing::Earth) => vec![WuXingRelation::BeingOvercome],
            // 水克火
            (WuXing::Water, WuXing::Fire) => vec![WuXingRelation::Overcoming],
            (WuXing::Fire, WuXing::Water) => vec![WuXingRelation::BeingOvercome],
            // 火克金
            (WuXing::Fire, WuXing::Metal) => vec![WuXingRelation::Overcoming],
            (WuXing::Metal, WuXing::Fire) => vec![WuXingRelation::BeingOvercome],
            // 金克木
            (WuXing::Metal, WuXing::Wood) => vec![WuXingRelation::Overcoming],
            (WuXing::Wood, WuXing::Metal) => vec![WuXingRelation::BeingOvercome],

            // 其他情况就是相同
            _ => vec![WuXingRelation::Same],
        }
    }

    fn from_relationship(&self, relationship: Self::Item) -> Option<Self> {
        match relationship {
            WuXingRelation::Generating => match self {
                WuXing::Wood => Some(WuXing::Fire),
                WuXing::Fire => Some(WuXing::Earth),
                WuXing::Earth => Some(WuXing::Metal),
                WuXing::Metal => Some(WuXing::Water),
                WuXing::Water => Some(WuXing::Wood),
            },
            WuXingRelation::BeingGenerated => match self {
                WuXing::Wood => Some(WuXing::Water),
                WuXing::Fire => Some(WuXing::Wood),
                WuXing::Earth => Some(WuXing::Fire),
                WuXing::Metal => Some(WuXing::Earth),
                WuXing::Water => Some(WuXing::Metal),
            },
            WuXingRelation::Overcoming => match self {
                WuXing::Wood => Some(WuXing::Earth),
                WuXing::Fire => Some(WuXing::Metal),
                WuXing::Earth => Some(WuXing::Water),
                WuXing::Metal => Some(WuXing::Wood),
                WuXing::Water => Some(WuXing::Fire),
            },
            WuXingRelation::BeingOvercome => match self {
                WuXing::Wood => Some(WuXing::Metal),
                WuXing::Fire => Some(WuXing::Water),
                WuXing::Earth => Some(WuXing::Wood),
                WuXing::Metal => Some(WuXing::Fire),
                WuXing::Water => Some(WuXing::Earth),
            },
            WuXingRelation::Same => match self {
                WuXing::Wood => Some(WuXing::Metal),
                WuXing::Fire => Some(WuXing::Water),
                WuXing::Earth => Some(WuXing::Wood),
                WuXing::Metal => Some(WuXing::Fire),
                WuXing::Water => Some(WuXing::Earth),
            }
        }
    }
}
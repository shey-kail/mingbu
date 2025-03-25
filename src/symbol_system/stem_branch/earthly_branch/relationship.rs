//! 地支关系相关类型的模块

use super::EarthlyBranch;
use crate::traits::{Index, Relationship, TripleRelationship};
use crate::basic::wuxing::WuXing;
use super::data::*;


/// 地支关系枚举
#[derive(Debug, Clone, PartialEq)]
pub enum EarthlyBranchRelationship {
    /// 相刑：寅刑巳 巳刑申 申刑寅、丑刑戌 戌刑未 未刑丑、子卯、辰午酉亥自刑
    Punishing,
    /// 被刑
    Punished,
    /// 三刑：寅巳申、丑戌未、子卯、辰午酉亥
    ThreePunishment,
    /// 相冲：子午、丑未、寅申、卯酉、辰戌、巳亥
    Opposition,
    /// 三合：寅午戌合火、亥卯未合木、巳酉丑合金、申子辰合水
    ThreeHarmony(WuXing),
    /// 半合
    HalfHarmony(WuXing),
    /// 拱合
    ArchHarmony(WuXing),
    /// 六合：子丑合土、寅亥合木、卯戌合火、辰酉合金、巳申合水、午未合火
    SixHarmony(WuXing),
    /// 三会：寅卯辰会木、巳午未会火、申酉戌会金、亥子丑会水
    ThreeMeeting(WuXing),
    /// 六害
    SixHarm,
    /// 破
    Break,
}

impl EarthlyBranch {
    /// 判断是否刑
    pub fn is_punishing(&self, other: &Self) -> bool {
        PUNISHING_PAIRS.iter().any(|(a, b)| (self, other) == (a, b))
    }
    pub fn is_punished(&self, other: &Self) -> bool {
        PUNISHING_PAIRS.iter().any(|(a, b)| (other, self) == (a, b))
    }
    /// 判断是否三刑
    pub fn is_three_punishment(&self, other: &Self) -> bool {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        PUNISHING_PAIRS.iter().any(|(a, b)| (branches[0], branches[1]) == (*a, *b))
    }
    /// 判断两个地支是否为相冲关系
    pub fn is_opposition(&self, other: &Self) -> bool {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        OPPOSITION_PAIRS.iter().any(|(a, b)| (branches[0], branches[1]) == (*a, *b))
    }
    /// 判断两个地支是否为六合关系
    pub fn is_six_harmony(&self, other: &Self) -> Option<WuXing> {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        SIX_HARMONY_PAIRS.iter()
            .find(|(a, b, _)| (branches[0], branches[1]) == (*a, *b))
            .map(|(_, _, wuxing)| *wuxing)
    }
    /// 判断三个地支是否为三合关系
    pub fn is_three_harmony(&self, second: &Self, third: &Self) -> Option<WuXing> {
        let mut branches = [*self, *second, *third];
        branches.sort_by_key(|b| b.index());
        THREE_HARMONY_GROUPS.iter()
            .find(|(a, b, c, _)| (branches[0], branches[1], branches[2]) == (*a, *b, *c))
            .map(|(_, _, _, wuxing)| *wuxing)
    }
    /// 判断两个地支是否为半合关系
    pub fn is_half_harmony(&self, other: &Self) -> Option<WuXing> {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        HALF_HARMONY_PAIRS.iter()
            .find(|(a, b, _)| (branches[0], branches[1]) == (*a, *b))
            .map(|(_, _, wuxing)| *wuxing)
    }
    /// 判断两个地支是否为拱合关系
    pub fn is_arch_harmony(&self, other: &Self) -> Option<WuXing> {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        ARCH_HARMONY_PAIRS.iter()
            .find(|(a, b, _)| (branches[0], branches[1]) == (*a, *b))
            .map(|(_, _, wuxing)| *wuxing)
    }
    /// 判断三个地支是否为三会关系
    pub fn is_three_meeting(&self, second: &Self, third: &Self) -> Option<WuXing> {
        let mut branches = [*self, *second, *third];
        branches.sort_by_key(|b| b.index());
        THREE_MEETING_GROUPS.iter()
            .find(|(a, b, c, _)| (branches[0], branches[1], branches[2]) == (*a, *b, *c))
            .map(|(_, _, _, wuxing)| *wuxing)
    }
    /// 判断两个地支是否为六害关系
    pub fn is_six_harm(&self, other: &Self) -> bool {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        SIX_HARM_PAIRS.iter().any(|(a, b)| (branches[0], branches[1]) == (*a, *b))
    }
    /// 判断两个地支是否为破关系
    pub fn is_break(&self, other: &Self) -> bool {
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        BREAK_PAIRS.iter().any(|(a, b)| (branches[0], branches[1]) == (*a, *b))
    }
}

impl Relationship for EarthlyBranch {
    type Item = EarthlyBranchRelationship;

    fn relationship_with(&self, other: &Self) -> Vec<Self::Item> {
        let mut relationships = Vec::new();

        // 检查相刑关系
        if self.is_punishing(other) {
            relationships.push(EarthlyBranchRelationship::Punishing);
        }
        if self.is_punished(other) {
            relationships.push(EarthlyBranchRelationship::Punished);
        }

        // 检查相冲关系
        if self.is_opposition(other) {
            relationships.push(EarthlyBranchRelationship::Opposition);
        }

        // 检查六合关系
        if let Some(wuxing) = self.is_six_harmony(other) {
            relationships.push(EarthlyBranchRelationship::SixHarmony(wuxing));
        }

        // 检查半合关系
        if let Some(wuxing) = self.is_half_harmony(other) {
            relationships.push(EarthlyBranchRelationship::HalfHarmony(wuxing));
        }

        // 检查拱合关系
        if let Some(wuxing) = self.is_arch_harmony(other) {
            relationships.push(EarthlyBranchRelationship::ArchHarmony(wuxing));
        }

        // 检查六害关系
        let mut branches = [*self, *other];
        branches.sort_by_key(|b| b.index());
        if SIX_HARM_PAIRS.iter().any(|(a, b)| (branches[0], branches[1]) == (*a, *b)) {
            relationships.push(EarthlyBranchRelationship::SixHarm);
        }

        // 检查破关系
        if BREAK_PAIRS.iter().any(|(a, b)| (branches[0], branches[1]) == (*a, *b)) {
            relationships.push(EarthlyBranchRelationship::Break);
        }
        relationships
    }

    fn from_relationship(&self, relationship: Self::Item) -> Option<Self> {
        match relationship {
            EarthlyBranchRelationship::Punishing => {
                PUNISHING_PAIRS.iter()
                    .find(|(a, _)| a == self)
                    .map(|(_, b)| *b)
            },
            EarthlyBranchRelationship::Punished => {
                PUNISHING_PAIRS.iter()
                    .find(|(_, b)| b == self)
                    .map(|(a, _)| *a)
            },
            EarthlyBranchRelationship::Opposition => {
                OPPOSITION_PAIRS.iter()
                    .find(|(a, b)| a == self || b == self)
                    .map(|(a, b)| if a == self { *b } else { *a })
            },
            EarthlyBranchRelationship::SixHarmony(_) => {
                SIX_HARMONY_PAIRS.iter()
                    .find(|(a, b, _)| a == self || b == self)
                    .map(|(a, b, _)| if a == self { *b } else { *a })
            },
            EarthlyBranchRelationship::HalfHarmony(_) => {
                HALF_HARMONY_PAIRS.iter()
                    .find(|(a, b, _)| a == self || b == self)
                    .map(|(a, b, _)| if a == self { *b } else { *a })
            },
            EarthlyBranchRelationship::ArchHarmony(_) => {
                ARCH_HARMONY_PAIRS.iter()
                    .find(|(a, b, _)| a == self || b == self)
                    .map(|(a, b, _)| if a == self { *b } else { *a })
            },
            EarthlyBranchRelationship::SixHarm => {
                SIX_HARM_PAIRS.iter()
                    .find(|(a, b)| a == self || b == self)
                    .map(|(a, b)| if a == self { *b } else { *a })
            },
            EarthlyBranchRelationship::Break => {
                BREAK_PAIRS.iter()
                    .find(|(a, b)| a == self || b == self)
                    .map(|(a, b)| if a == self { *b } else { *a })
            },
            _ => None,
        }
    }
}


/// 三方关系特征，用于描述事物间的三方关系
impl TripleRelationship for EarthlyBranch {
    type Item = EarthlyBranchRelationship;
    /// 获取与两个目标的关系（三方关系）
    fn relationship_with_triple(&self, second: &Self, third: &Self) -> Option<Self::Item> {
        let mut branches = [*self, *second, *third];
        branches.sort_by_key(|b| b.index());

        // 检查三合关系
        if let Some(wuxing) = self.is_three_harmony(second, third) {
            return Some(EarthlyBranchRelationship::ThreeHarmony(wuxing));
        }

        // 检查三会关系
        if let Some(wuxing) = self.is_three_meeting(second, third) {
            return Some(EarthlyBranchRelationship::ThreeMeeting(wuxing));
        }

        // 检查三刑关系
        if self.is_three_punishment(second) && self.is_three_punishment(third) {
            return Some(EarthlyBranchRelationship::ThreePunishment);
        }

        None
    }

    fn from_relationship_triple(&self, relationship: Self::Item) -> (Option<Self>, Option<Self>) where Self: Sized {
        match relationship {
            EarthlyBranchRelationship::ThreeHarmony(wuxing) => {
                for &(a, b, c, w) in THREE_HARMONY_GROUPS.iter() {
                    if *self == a && w == wuxing {
                        return (Some(b), Some(c));
                    } else if *self == b && w == wuxing {
                        return (Some(a), Some(c));
                    } else if *self == c && w == wuxing {
                        return (Some(a), Some(b));
                    }
                }
            },
            EarthlyBranchRelationship::ThreeMeeting(wuxing) => {
                for &(a, b, c, w) in THREE_MEETING_GROUPS.iter() {
                    if *self == a && w == wuxing {
                        return (Some(b), Some(c));
                    } else if *self == b && w == wuxing {
                        return (Some(a), Some(c));
                    } else if *self == c && w == wuxing {
                        return (Some(a), Some(b));
                    }
                }
            },
            EarthlyBranchRelationship::ThreePunishment => {
                for &(a, b) in PUNISHING_PAIRS.iter() {
                    if *self == a {
                        return (Some(b), Some(b));
                    } else if *self == b {
                        return (Some(a), Some(a));
                    }
                }
            },
            _ => {}
        }
        (None, None)
    }
}

//! 天干关系相关类型的模块

use super::HeavenlyStem;

/// 天干关系枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeavenlyStemRelationship {
    /// 五合：甲己合化土、乙庚合化金、丙辛合化水、丁壬合化木、戊癸合化火
    FiveHarmony,
    /// 六合：暂时保留
    SixHarmony,
    /// 三会：暂时保留
    ThreeHarmony,
}

impl HeavenlyStem {
    /// 判断两个天干是否为五合关系
    pub fn is_five_harmony(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (HeavenlyStem::Jia, HeavenlyStem::Ji)
            | (HeavenlyStem::Ji, HeavenlyStem::Jia)
            | (HeavenlyStem::Yi, HeavenlyStem::Geng)
            | (HeavenlyStem::Geng, HeavenlyStem::Yi)
            | (HeavenlyStem::Bing, HeavenlyStem::Xin)
            | (HeavenlyStem::Xin, HeavenlyStem::Bing)
            | (HeavenlyStem::Ding, HeavenlyStem::Ren)
            | (HeavenlyStem::Ren, HeavenlyStem::Ding)
            | (HeavenlyStem::Wu, HeavenlyStem::Gui)
            | (HeavenlyStem::Gui, HeavenlyStem::Wu)
        )
    }
}
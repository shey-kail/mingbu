//! 天干关系相关类型的模块

use super::HeavenlyStem;
use crate::traits::Relationship;
use crate::traits::yinyang_wuxing::{WuXingTrait, YinYangTrait};
use crate::basic::{YinYang, WuXing, WuXingRelation};

// 天干关系枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeavenlyStemRelationship {
    /// 五合
    Harmony(WuXing),  
}

/// 十神枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenGods {
    /// 正印：生我者
    ZhengYin,
    /// 偏印：生我者
    PianYin,
    /// 正官：克我者
    ZhengGuan,
    /// 七杀：克我者
    QiSha,
    /// 正财：我生者
    ZhengCai,
    /// 偏财：我生者
    PianCai,
    /// 食神：我克者
    ShiShen,
    /// 伤官：我克者
    ShangGuan,
    /// 比肩：同我者
    BiJian,
    /// 劫财：同我者
    JieCai
}

impl Relationship for HeavenlyStem {
    type Item = HeavenlyStemRelationship;

    fn relationship_with(&self, other: &Self) -> Vec<Self::Item> {
        match (self, other) {
            // 甲己合土
            (HeavenlyStem::Jia, HeavenlyStem::Ji)
            | (HeavenlyStem::Ji, HeavenlyStem::Jia) => {
                vec![
                    HeavenlyStemRelationship::Harmony(WuXing::Earth)
                ]
            },
            // 乙庚合金
            (HeavenlyStem::Yi, HeavenlyStem::Geng)
            | (HeavenlyStem::Geng, HeavenlyStem::Yi) => {
                vec![
                    HeavenlyStemRelationship::Harmony(WuXing::Metal)
                ]
            },
            // 丙辛合水
            (HeavenlyStem::Bing, HeavenlyStem::Xin)
            | (HeavenlyStem::Xin, HeavenlyStem::Bing) => {
                vec![
                    HeavenlyStemRelationship::Harmony(WuXing::Water)
                ]
            },
            // 丁壬合木
            (HeavenlyStem::Ding, HeavenlyStem::Ren)
            | (HeavenlyStem::Ren, HeavenlyStem::Ding) => {
                vec![
                    HeavenlyStemRelationship::Harmony(WuXing::Wood)
                ] 
            },
            // 戊癸合火
            (HeavenlyStem::Wu, HeavenlyStem::Gui)
            | (HeavenlyStem::Gui, HeavenlyStem::Wu) => {
                vec![
                    HeavenlyStemRelationship::Harmony(WuXing::Fire)
                ]
            },
            // 其他情况，返回空
            _ => vec![]
        }
    }

    // 从关系反向推导出对应的目标
    fn from_relationship(&self, relationship: Self::Item) -> Option<Self> {
        // 如果是天干五合的话，再进行下面的判断
        if let HeavenlyStemRelationship::Harmony(_) = relationship {
            match self {
                HeavenlyStem::Jia => Some(HeavenlyStem::Ji), 
                HeavenlyStem::Ji => Some(HeavenlyStem::Jia),
                HeavenlyStem::Yi => Some(HeavenlyStem::Geng),
                HeavenlyStem::Geng => Some(HeavenlyStem::Yi),
                HeavenlyStem::Bing => Some(HeavenlyStem::Xin),
                HeavenlyStem::Xin => Some(HeavenlyStem::Bing),
                HeavenlyStem::Ding => Some(HeavenlyStem::Ren),
                HeavenlyStem::Ren => Some(HeavenlyStem::Ding),
                HeavenlyStem::Wu => Some(HeavenlyStem::Gui),
                HeavenlyStem::Gui => Some(HeavenlyStem::Wu),
            }
        } else {
            None
        }
    }
}

impl HeavenlyStem {
    /// 判断两个天干是否为五合关系
    pub fn is_harmony(&self, other: &Self) -> bool {
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

    pub fn from_yinyang_wuxing(yinyang: YinYang, wuxing: WuXing) -> Self {
        match (yinyang, wuxing) {
            (YinYang::Yang, WuXing::Wood) => HeavenlyStem::Jia,
            (YinYang::Yang, WuXing::Fire) => HeavenlyStem::Bing,
            (YinYang::Yang, WuXing::Earth) => HeavenlyStem::Wu, 
            (YinYang::Yang, WuXing::Metal) => HeavenlyStem::Geng,
            (YinYang::Yang, WuXing::Water) => HeavenlyStem::Ren,

            (YinYang::Yin, WuXing::Wood) => HeavenlyStem::Yi,
            (YinYang::Yin, WuXing::Fire) => HeavenlyStem::Ding,
            (YinYang::Yin, WuXing::Earth) => HeavenlyStem::Ji,
            (YinYang::Yin, WuXing::Metal) => HeavenlyStem::Xin,
            (YinYang::Yin, WuXing::Water) => HeavenlyStem::Gui
        }
    }
    pub fn ten_gods(&self, other: &Self) -> TenGods {
        let self_wuxing = self.wuxing();
        let other_wuxing = other.wuxing();
        let wuxing_relationship_vec = self_wuxing.relationship_with(&other_wuxing);
        // 五行关系肯定不会有多个，也不会是空，所以这里不检查vec的长度
        let wuxing_relationship = wuxing_relationship_vec[0];
        let is_yinyang_same = self.yinyang() == other.yinyang();

        match (wuxing_relationship, is_yinyang_same) {
            // 正印, 生我者
            (WuXingRelation::BeingGenerated, false) => TenGods::ZhengYin,
            // 偏印, 生我者
            (WuXingRelation::BeingGenerated, true) => TenGods::PianYin,
            // 正官, 克我者
            (WuXingRelation::BeingOvercome, false) => TenGods::ZhengGuan,
            // 七杀, 克我者
            (WuXingRelation::BeingOvercome, true) => TenGods::QiSha,
            // 正财, 我克者
            (WuXingRelation::Overcoming, false) => TenGods::ZhengCai,
            // 偏财, 我克者
            (WuXingRelation::Overcoming, true) => TenGods::PianCai,
            // 比肩, 同我者
            (WuXingRelation::Same, true) => TenGods::BiJian,
            // 劫财, 同我者
            (WuXingRelation::Same, false) => TenGods::JieCai,
            // 食神, 我生者
            (WuXingRelation::Generating, true) => TenGods::ShiShen,
            // 伤官, 我生者
            (WuXingRelation::Generating, false) => TenGods::ShangGuan,
        }
    }
}
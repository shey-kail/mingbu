//! 天干关系相关类型的模块

use super::HeavenlyStem;
use crate::traits::Relationship;
use crate::basic::WuXingRelation;
use crate::basic::{YinYang, WuXing};

/// 天干关系枚举
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
    type Item = TenGods;

    fn relationship_with(&self, other: &Self) -> Self::Item {
        let self_wuxing = self.wuxing();
        let other_wuxing = other.wuxing();
        let wuxing_relationship = self_wuxing.relationship_with(&other_wuxing);
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
            (WuXingRelation::Generating, false) => TenGods::ShiShen,
            // 伤官, 我生者
            (WuXingRelation::Generating, true) => TenGods::ShangGuan,
        }
    }

    fn from_relationship(&self, relationship: Self::Item) -> Self {
        let (wuxing, yinyang) = match relationship {
            // 正印, 生我者
            TenGods::ZhengYin =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::BeingGenerated
                    ),
                self.yinyang().opposite()
            ),
            // 偏印, 生我者
            TenGods::PianYin =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::BeingGenerated
                    ),
                self.yinyang()
            ),
            // 正官, 克我者
            TenGods::ZhengGuan =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::BeingOvercome
                    ),
                self.yinyang().opposite()
            ),
            // 七杀, 克我者
            TenGods::QiSha =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::BeingOvercome
                    ),
                self.yinyang()
            ),
            // 正财, 我克者
            TenGods::ZhengCai =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::Overcoming
                    ),
                self.yinyang().opposite()
            ),
            // 偏财, 我克者
            TenGods::PianCai =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::Overcoming
                    ),
                self.yinyang()
            ),
            // 食神, 我生者
            TenGods::ShiShen =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::Generating
                    ),
                self.yinyang().opposite()
            ),
            // 伤官, 我生者
            TenGods::ShangGuan =>  (
                self.wuxing().from_relationship(
                        WuXingRelation::Generating
                    ),
                self.yinyang()
            ),
            // 比肩, 同我者
            TenGods::BiJian =>  (
                self.wuxing(),
                self.yinyang()
            ),
            // 劫财, 同我者
            TenGods::JieCai =>  (
                self.wuxing(),
                self.yinyang().opposite()  
            )
       }; 
       HeavenlyStem::from_yinyang_wuxing(yinyang, wuxing)
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
}
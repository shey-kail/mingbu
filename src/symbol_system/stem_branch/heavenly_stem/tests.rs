//! 天干模块的测试代码

#[cfg(test)]
mod tests {
    use super::super::{HeavenlyStem, relationship::HeavenlyStemRelationship};
    use crate::basic::{YinYang, WuXing};
    use crate::traits::{ChineseName, Index, Iter, Relationship};
    use crate::traits::yinyang_wuxing::{WuXingTrait, YinYangTrait};
    use super::super::relationship::TenGods;

    #[test]
    fn test_yinyang() {
        assert_eq!(HeavenlyStem::Jia.yinyang(), YinYang::Yang);
        assert_eq!(HeavenlyStem::Yi.yinyang(), YinYang::Yin);
        assert_eq!(HeavenlyStem::Bing.yinyang(), YinYang::Yang);
        assert_eq!(HeavenlyStem::Ding.yinyang(), YinYang::Yin);
        assert_eq!(HeavenlyStem::Wu.yinyang(), YinYang::Yang);
        assert_eq!(HeavenlyStem::Ji.yinyang(), YinYang::Yin);
        assert_eq!(HeavenlyStem::Geng.yinyang(), YinYang::Yang);
        assert_eq!(HeavenlyStem::Xin.yinyang(), YinYang::Yin);
        assert_eq!(HeavenlyStem::Ren.yinyang(), YinYang::Yang);
        assert_eq!(HeavenlyStem::Gui.yinyang(), YinYang::Yin);
    }

    #[test]
    fn test_wuxing() {
        assert_eq!(HeavenlyStem::Jia.wuxing(), WuXing::Wood);
        assert_eq!(HeavenlyStem::Yi.wuxing(), WuXing::Wood);
        assert_eq!(HeavenlyStem::Bing.wuxing(), WuXing::Fire);
        assert_eq!(HeavenlyStem::Ding.wuxing(), WuXing::Fire);
        assert_eq!(HeavenlyStem::Wu.wuxing(), WuXing::Earth);
        assert_eq!(HeavenlyStem::Ji.wuxing(), WuXing::Earth);
        assert_eq!(HeavenlyStem::Geng.wuxing(), WuXing::Metal);
        assert_eq!(HeavenlyStem::Xin.wuxing(), WuXing::Metal);
        assert_eq!(HeavenlyStem::Ren.wuxing(), WuXing::Water);
        assert_eq!(HeavenlyStem::Gui.wuxing(), WuXing::Water);
    }

    #[test]
    fn test_chinese_name() {
        assert_eq!(HeavenlyStem::Jia.chinese_name(), "甲");
        assert_eq!(HeavenlyStem::Yi.chinese_name(), "乙");
        assert_eq!(HeavenlyStem::Bing.chinese_name(), "丙");
        assert_eq!(HeavenlyStem::Ding.chinese_name(), "丁");
        assert_eq!(HeavenlyStem::Wu.chinese_name(), "戊");
        assert_eq!(HeavenlyStem::Ji.chinese_name(), "己");
        assert_eq!(HeavenlyStem::Geng.chinese_name(), "庚");
        assert_eq!(HeavenlyStem::Xin.chinese_name(), "辛");
        assert_eq!(HeavenlyStem::Ren.chinese_name(), "壬");
        assert_eq!(HeavenlyStem::Gui.chinese_name(), "癸");
    }

    #[test]
    fn test_index() {
        // 测试从索引创建天干
        assert_eq!(HeavenlyStem::from_index(1), HeavenlyStem::Jia);
        assert_eq!(HeavenlyStem::from_index(2), HeavenlyStem::Yi);
        assert_eq!(HeavenlyStem::from_index(10), HeavenlyStem::Gui);
        assert_eq!(HeavenlyStem::from_index(11), HeavenlyStem::Jia);

        // 测试获取天干的索引
        assert_eq!(HeavenlyStem::Jia.index(), 1);
        assert_eq!(HeavenlyStem::Yi.index(), 2);
        assert_eq!(HeavenlyStem::Gui.index(), 10);
    }

    #[test]
    fn test_iter_next() {
        assert_eq!(HeavenlyStem::Jia.next(), HeavenlyStem::Yi);
        assert_eq!(HeavenlyStem::Yi.next(), HeavenlyStem::Bing);
        assert_eq!(HeavenlyStem::Gui.next(), HeavenlyStem::Jia);
    }

    #[test]
    fn test_iter_prev() {
        println!("{:?}", HeavenlyStem::Jia.prev());
        println!("{:?}", HeavenlyStem::Yi.prev());
        println!("{:?}", HeavenlyStem::Gui.prev());
        assert_eq!(HeavenlyStem::Jia.prev(), HeavenlyStem::Gui);
        assert_eq!(HeavenlyStem::Yi.prev(), HeavenlyStem::Jia);
        assert_eq!(HeavenlyStem::Gui.prev(), HeavenlyStem::Ren);
    }

    #[test]
    fn test_ten_gods_relationship() {
        // 测试比肩关系（同我者）
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Jia), TenGods::BiJian);
        
        // 测试劫财关系（同我者）
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Yi), TenGods::JieCai);

        // 测试食神关系（我克者）
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Bing), TenGods::ShiShen);

        // 测试伤官关系（我克者）
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Ding), TenGods::ShangGuan);

        // 测试正财关系（我克者）.ten_gods(
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Ji), TenGods::ZhengCai);

        // 测试偏财关系（我克者）.ten_gods(
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Wu), TenGods::PianCai);
        
        // 测试正官关系（克我者）.ten_gods(
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Xin), TenGods::ZhengGuan);

        // 测试七杀关系（克我者）.ten_gods(
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Geng), TenGods::QiSha);

        // 测试正印关系（生我者）.ten_gods(
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Gui), TenGods::ZhengYin);

        // 测试偏印关系（生我者）.ten_gods(
        assert_eq!(HeavenlyStem::Jia.ten_gods(&HeavenlyStem::Ren), TenGods::PianYin);
    }

    #[test]
    fn test_harmony() {
        // 测试五合关系
        assert!(HeavenlyStem::Jia.is_harmony(&HeavenlyStem::Ji));
        assert!(HeavenlyStem::Ji.is_harmony(&HeavenlyStem::Jia));
        assert!(HeavenlyStem::Yi.is_harmony(&HeavenlyStem::Geng));
        assert!(HeavenlyStem::Geng.is_harmony(&HeavenlyStem::Yi));
        assert!(HeavenlyStem::Bing.is_harmony(&HeavenlyStem::Xin));
        assert!(HeavenlyStem::Xin.is_harmony(&HeavenlyStem::Bing));
        assert!(HeavenlyStem::Ding.is_harmony(&HeavenlyStem::Ren));
        assert!(HeavenlyStem::Ren.is_harmony(&HeavenlyStem::Ding));
        assert!(HeavenlyStem::Wu.is_harmony(&HeavenlyStem::Gui));
        assert!(HeavenlyStem::Gui.is_harmony(&HeavenlyStem::Wu));

        // 测试非五合关系
        assert!(!HeavenlyStem::Jia.is_harmony(&HeavenlyStem::Yi));
        assert!(!HeavenlyStem::Bing.is_harmony(&HeavenlyStem::Wu));
        assert!(!HeavenlyStem::Ding.is_harmony(&HeavenlyStem::Ji));
    }

    #[test]
    fn test_from_yinyang_wuxing() {
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yang, WuXing::Wood), HeavenlyStem::Jia);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yin, WuXing::Wood), HeavenlyStem::Yi);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yang, WuXing::Fire), HeavenlyStem::Bing);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yin, WuXing::Fire), HeavenlyStem::Ding);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yang, WuXing::Earth), HeavenlyStem::Wu);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yin, WuXing::Earth), HeavenlyStem::Ji);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yang, WuXing::Metal), HeavenlyStem::Geng);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yin, WuXing::Metal), HeavenlyStem::Xin);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yang, WuXing::Water), HeavenlyStem::Ren);
        assert_eq!(HeavenlyStem::from_yinyang_wuxing(YinYang::Yin, WuXing::Water), HeavenlyStem::Gui);
    }

    #[test]
    fn test_relationship_with() {
        // 测试五合关系
        assert_eq!(HeavenlyStem::Jia.relationship_with(&HeavenlyStem::Ji)[0], HeavenlyStemRelationship::Harmony(WuXing::Earth));
        assert_eq!(HeavenlyStem::Yi.relationship_with(&HeavenlyStem::Geng)[0], HeavenlyStemRelationship::Harmony(WuXing::Metal));
        assert_eq!(HeavenlyStem::Bing.relationship_with(&HeavenlyStem::Xin)[0], HeavenlyStemRelationship::Harmony(WuXing::Water));
        assert_eq!(HeavenlyStem::Ding.relationship_with(&HeavenlyStem::Ren)[0], HeavenlyStemRelationship::Harmony(WuXing::Wood));
        assert_eq!(HeavenlyStem::Wu.relationship_with(&HeavenlyStem::Gui)[0], HeavenlyStemRelationship::Harmony(WuXing::Fire));

        // 测试非五合关系
        assert_eq!(HeavenlyStem::Jia.relationship_with(&HeavenlyStem::Yi).len(), 0);
        assert_eq!(HeavenlyStem::Bing.relationship_with(&HeavenlyStem::Wu).len(), 0);
    }

    #[test]
    fn test_from_relationship() {
        // 测试从五合关系反推天干
        assert_eq!(HeavenlyStem::Jia.from_relationship(HeavenlyStemRelationship::Harmony(WuXing::Earth)), Some(HeavenlyStem::Ji));
        assert_eq!(HeavenlyStem::Yi.from_relationship(HeavenlyStemRelationship::Harmony(WuXing::Metal)), Some(HeavenlyStem::Geng));
        assert_eq!(HeavenlyStem::Bing.from_relationship(HeavenlyStemRelationship::Harmony(WuXing::Water)), Some(HeavenlyStem::Xin));
        assert_eq!(HeavenlyStem::Ding.from_relationship(HeavenlyStemRelationship::Harmony(WuXing::Wood)), Some(HeavenlyStem::Ren));
        assert_eq!(HeavenlyStem::Wu.from_relationship(HeavenlyStemRelationship::Harmony(WuXing::Fire)), Some(HeavenlyStem::Gui));
    }
}

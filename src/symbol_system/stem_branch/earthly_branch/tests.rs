//! 地支相关类型的测试模块

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::basic::{YinYang, WuXing};
    use crate::traits::{Index, ChineseName, Iter};
    use crate::traits::yinyang_wuxing::{WuXingTrait, YinYangTrait};

    #[test]
    fn test_yinyang() {
        // 测试阳支
        assert_eq!(EarthlyBranch::Zi.yinyang(), YinYang::Yang);
        assert_eq!(EarthlyBranch::Wu.yinyang(), YinYang::Yang);
        assert_eq!(EarthlyBranch::Chen.yinyang(), YinYang::Yang);
        assert_eq!(EarthlyBranch::You.yinyang(), YinYang::Yang);
        assert_eq!(EarthlyBranch::Shen.yinyang(), YinYang::Yang);
        assert_eq!(EarthlyBranch::Yin.yinyang(), YinYang::Yang);

        // 测试阴支
        assert_eq!(EarthlyBranch::Chou.yinyang(), YinYang::Yin);
        assert_eq!(EarthlyBranch::Wei.yinyang(), YinYang::Yin);
        assert_eq!(EarthlyBranch::Si.yinyang(), YinYang::Yin);
        assert_eq!(EarthlyBranch::Hai.yinyang(), YinYang::Yin);
        assert_eq!(EarthlyBranch::Mao.yinyang(), YinYang::Yin);
        assert_eq!(EarthlyBranch::Xu.yinyang(), YinYang::Yin);
    }

    #[test]
    fn test_wuxing() {
        // 测试木
        assert_eq!(EarthlyBranch::Yin.wuxing(), WuXing::Wood);
        assert_eq!(EarthlyBranch::Mao.wuxing(), WuXing::Wood);

        // 测试火
        assert_eq!(EarthlyBranch::Si.wuxing(), WuXing::Fire);
        assert_eq!(EarthlyBranch::Wu.wuxing(), WuXing::Fire);

        // 测试土
        assert_eq!(EarthlyBranch::Chen.wuxing(), WuXing::Earth);
        assert_eq!(EarthlyBranch::Xu.wuxing(), WuXing::Earth);
        assert_eq!(EarthlyBranch::Chou.wuxing(), WuXing::Earth);
        assert_eq!(EarthlyBranch::Wei.wuxing(), WuXing::Earth);

        // 测试金
        assert_eq!(EarthlyBranch::Shen.wuxing(), WuXing::Metal);
        assert_eq!(EarthlyBranch::You.wuxing(), WuXing::Metal);

        // 测试水
        assert_eq!(EarthlyBranch::Hai.wuxing(), WuXing::Water);
        assert_eq!(EarthlyBranch::Zi.wuxing(), WuXing::Water);
    }

    #[test]
    fn test_chinese_name() {
        assert_eq!(EarthlyBranch::Zi.chinese_name(), "子");
        assert_eq!(EarthlyBranch::Chou.chinese_name(), "丑");
        assert_eq!(EarthlyBranch::Yin.chinese_name(), "寅");
        assert_eq!(EarthlyBranch::Mao.chinese_name(), "卯");
        assert_eq!(EarthlyBranch::Chen.chinese_name(), "辰");
        assert_eq!(EarthlyBranch::Si.chinese_name(), "巳");
        assert_eq!(EarthlyBranch::Wu.chinese_name(), "午");
        assert_eq!(EarthlyBranch::Wei.chinese_name(), "未");
        assert_eq!(EarthlyBranch::Shen.chinese_name(), "申");
        assert_eq!(EarthlyBranch::You.chinese_name(), "酉");
        assert_eq!(EarthlyBranch::Xu.chinese_name(), "戌");
        assert_eq!(EarthlyBranch::Hai.chinese_name(), "亥");
    }

    #[test]
    fn test_index() {
        // 测试从索引创建地支
        assert_eq!(EarthlyBranch::from_index(1), EarthlyBranch::Zi);
        assert_eq!(EarthlyBranch::from_index(13), EarthlyBranch::Zi); // 测试循环

        // 测试获取地支索引
        assert_eq!(EarthlyBranch::Zi.index(), 1);
        assert_eq!(EarthlyBranch::Chou.index(), 2);
        assert_eq!(EarthlyBranch::Yin.index(), 3);
        assert_eq!(EarthlyBranch::Mao.index(), 4);
        assert_eq!(EarthlyBranch::Chen.index(), 5);
        assert_eq!(EarthlyBranch::Si.index(), 6);
        assert_eq!(EarthlyBranch::Wu.index(), 7);
        assert_eq!(EarthlyBranch::Wei.index(), 8);
        assert_eq!(EarthlyBranch::Shen.index(), 9);
        assert_eq!(EarthlyBranch::You.index(), 10);
        assert_eq!(EarthlyBranch::Xu.index(), 11);
        assert_eq!(EarthlyBranch::Hai.index(), 12);
    }

    #[test]
    fn test_iter() {
        // 测试next
        assert_eq!(EarthlyBranch::Zi.next(), EarthlyBranch::Chou);
        assert_eq!(EarthlyBranch::Hai.next(), EarthlyBranch::Zi); // 测试循环

        // 测试prev
        assert_eq!(EarthlyBranch::Chou.prev(), EarthlyBranch::Zi);
        assert_eq!(EarthlyBranch::Zi.prev(), EarthlyBranch::Hai); // 测试循环
    }

    #[test]
    fn test_relationship() {
        // 测试相刑关系
        assert!(EarthlyBranch::Yin.is_punishing(&EarthlyBranch::Si));
        assert!(EarthlyBranch::Si.is_punishing(&EarthlyBranch::Shen));
        assert!(EarthlyBranch::Shen.is_punishing(&EarthlyBranch::Yin));

        // 测试相冲关系
        assert!(EarthlyBranch::Zi.is_opposition(&EarthlyBranch::Wu));
        assert!(EarthlyBranch::Mao.is_opposition(&EarthlyBranch::You));

        // 测试三合关系
        assert_eq!(
            EarthlyBranch::Yin.is_three_harmony(&EarthlyBranch::Wu, &EarthlyBranch::Xu),
            Some(WuXing::Fire)
        );

        // 测试六合关系
        assert_eq!(
            EarthlyBranch::Zi.is_six_harmony(&EarthlyBranch::Chou),
            Some(WuXing::Earth)
        );

        // 测试三会关系
        assert_eq!(
            EarthlyBranch::Yin.is_three_meeting(&EarthlyBranch::Mao, &EarthlyBranch::Chen),
            Some(WuXing::Wood)
        );

        // 测试六害关系
        assert!(EarthlyBranch::Zi.is_six_harm(&EarthlyBranch::Wei));

        // 测试破关系
        assert!(EarthlyBranch::Yin.is_break(&EarthlyBranch::Wei));
    }

    #[test]
    fn test_hidden_stem() {
        use super::super::super::HeavenlyStem;

        // 测试地支藏干
        let zi_stems = EarthlyBranch::Zi.hidden_stems();
        assert_eq!(zi_stems.len(), 1);
        assert_eq!(zi_stems[0], HeavenlyStem::Gui);

        let chou_stems = EarthlyBranch::Chou.hidden_stems();
        assert_eq!(chou_stems.len(), 3);
        assert_eq!(chou_stems[0], HeavenlyStem::Ji);
        assert_eq!(chou_stems[1], HeavenlyStem::Gui);
        assert_eq!(chou_stems[2], HeavenlyStem::Xin);

        // 测试主气、中气、余气
        assert_eq!(EarthlyBranch::Zi.main_stem(), HeavenlyStem::Gui);
        assert_eq!(EarthlyBranch::Chou.middle_stem(), Some(HeavenlyStem::Gui));
        assert_eq!(EarthlyBranch::Chou.residual_stem(), Some(HeavenlyStem::Xin));
    }
}
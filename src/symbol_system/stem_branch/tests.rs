//! 干支模块的测试

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::traits::{ChineseName, Iter};
    use crate::symbol_system::stem_branch::sexagesimal_cycle::*;

    #[test]
    fn test_sexagesimal_cycle() {
        // 测试六十甲子的初始化
        let cycle = SexagesimalCycle::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(cycle.chinese_name(), "甲子");

        // 测试六十甲子的迭代
        let mut next = cycle.next();
        assert_eq!(next.chinese_name(), "乙丑");
        next = next.next();
        assert_eq!(next.chinese_name(), "丙寅");

        // 测试六十甲子的逆向迭代
        let mut prev = cycle.prev();
        assert_eq!(prev.chinese_name(), "癸亥");
        prev = prev.prev();
        assert_eq!(prev.chinese_name(), "壬戌");
    }

    #[test]
    fn test_twelve_stages() {
        // 测试帝旺对应关系
        let stem = HeavenlyStem::Jia;
        let diwang_branch = EarthlyBranch::Mao;
        let stages = TwelveStages::get_twelve_stages(stem, diwang_branch);
        assert_eq!(stages.chinese_name(), "帝旺");

        // 测试长生位置计算
        let stages = TwelveStages::get_twelve_stages(stem, EarthlyBranch::Hai);
        assert_eq!(stages.chinese_name(), "长生");

        // 测试十二长生的迭代
        let stages = TwelveStages::from_chinese_name("长生");
        let next = stages.next();
        assert_eq!(next.chinese_name(), "沐浴");
        let prev = stages.prev();
        assert_eq!(prev.chinese_name(), "养");

        // 测试根据天干和十二长生获得地支
        let stages = TwelveStages::from_chinese_name("长生");
        let branch = stages.get_earthly_branch(HeavenlyStem::Jia);
        assert_eq!(branch, EarthlyBranch::Hai);

        // 测试根据地支和十二长生获得天干
        let stages = TwelveStages::from_chinese_name("帝旺");
        let stem = stages.get_heavenly_stem(EarthlyBranch::Mao);
        assert_eq!(stem, HeavenlyStem::Jia);
    }
}
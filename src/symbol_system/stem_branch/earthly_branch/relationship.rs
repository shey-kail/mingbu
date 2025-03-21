//! 地支关系相关类型的模块

use super::EarthlyBranch;

/// 地支关系枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarthlyBranchRelationship {
    /// 三合：寅午戌合火、亥卯未合木、巳酉丑合金、申子辰合水
    ThreeHarmony,
    /// 六合：子丑合土、寅亥合木、卯戌合火、辰酉合金、巳申合水、午未合火
    SixHarmony,
    /// 相刑：寅巳申、丑戌未、子卯、辰午酉亥
    Punishment,
    /// 相冲：子午、丑未、寅申、卯酉、辰戌、巳亥
    Opposition,
}

impl EarthlyBranch {
    /// 判断两个地支是否为六合关系
    pub fn is_six_harmony(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (EarthlyBranch::Zi, EarthlyBranch::Chou)
            | (EarthlyBranch::Chou, EarthlyBranch::Zi)
            | (EarthlyBranch::Yin, EarthlyBranch::Hai)
            | (EarthlyBranch::Hai, EarthlyBranch::Yin)
            | (EarthlyBranch::Mao, EarthlyBranch::Xu)
            | (EarthlyBranch::Xu, EarthlyBranch::Mao)
            | (EarthlyBranch::Chen, EarthlyBranch::You)
            | (EarthlyBranch::You, EarthlyBranch::Chen)
            | (EarthlyBranch::Si, EarthlyBranch::Shen)
            | (EarthlyBranch::Shen, EarthlyBranch::Si)
            | (EarthlyBranch::Wu, EarthlyBranch::Wei)
            | (EarthlyBranch::Wei, EarthlyBranch::Wu)
        )
    }

    /// 判断两个地支是否为相冲关系
    pub fn is_opposition(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (EarthlyBranch::Zi, EarthlyBranch::Wu)
            | (EarthlyBranch::Wu, EarthlyBranch::Zi)
            | (EarthlyBranch::Chou, EarthlyBranch::Wei)
            | (EarthlyBranch::Wei, EarthlyBranch::Chou)
            | (EarthlyBranch::Yin, EarthlyBranch::Shen)
            | (EarthlyBranch::Shen, EarthlyBranch::Yin)
            | (EarthlyBranch::Mao, EarthlyBranch::You)
            | (EarthlyBranch::You, EarthlyBranch::Mao)
            | (EarthlyBranch::Chen, EarthlyBranch::Xu)
            | (EarthlyBranch::Xu, EarthlyBranch::Chen)
            | (EarthlyBranch::Si, EarthlyBranch::Hai)
            | (EarthlyBranch::Hai, EarthlyBranch::Si)
        )
    }
}
//! 地支关系相关类型的模块

use super::EarthlyBranch;
use crate::traits::{Relationship, TripleRelationship};

/// 地支关系枚举
enum EarthlyBranchRelationship {
    /// 相刑：寅巳申、丑戌未、子卯、辰午酉亥
    Punishment,
    /// 相冲：子午、丑未、寅申、卯酉、辰戌、巳亥
    Opposition,
    /// 三合：寅午戌合火、亥卯未合木、巳酉丑合金、申子辰合水
    ThreeHarmony,
    /// 半合
    HalfHarmony,
    /// 六合：子丑合土、寅亥合木、卯戌合火、辰酉合金、巳申合水、午未合火
    SixHarmony,
    /// 三会：寅卯辰会木、巳午未会火、申酉戌会金、亥子丑会水
    ThreeMeeting,
    /// 六害
    SixHarm,
    /// 破
    Break,
}

impl EarthlyBranch {
    /// 判断两个地址是否相刑
    fn is_punishment(&self, other: &Self) -> bool {
        matches!(
            [self, other].sort_by_key(|x| x.index()),
            [EarthlyBranch::Yin, EarthlyBranch::Si]
            | (EarthlyBranch::Si, EarthlyBranch::Yin)
            | (EarthlyBranch::Si, EarthlyBranch::Shen)
            | (EarthlyBranch::Shen, EarthlyBranch::Si)
            | (EarthlyBranch::Shen, EarthlyBranch::You)
            | (EarthlyBranch::Chou, EarthlyBranch::Xu)

        )
    }
    /// 判断两个地支是否为六合关系
    fn is_six_harmony(&self, other: &Self) -> bool {
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
    fn is_opposition(&self, other: &Self) -> bool {
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


/// 关系特征，用于描述事物间的双方关系
impl Relationship for EarthlyBranch {
    type Item = EarthlyBranch;
    /// 获取我对目标的关系（双方关系）
    fn relationship_with(&self, other: &Self) -> Self::Item {

    }
    /// 根据给定的关系反向推导出对应的目标
    fn from_relationship(&self, relationship: Self::Item) -> Self {

    }
}

/// 三方关系特征，用于描述事物间的三方关系
impl TripleRelationship for EarthlyBranch {
    type Item = EarthlyBranch;
    /// 获取与两个目标的关系（三方关系）
    fn relationship_with_triple(&self, second: &Self, third: &Self) -> Self::Item {

    }
}

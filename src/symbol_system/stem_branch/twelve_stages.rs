//! 十二长生相关类型的模块

/// 十二长生枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwelveStages {
    /// 长生
    Birth,
    /// 沐浴
    Bath,
    /// 冠带
    Crown,
    /// 临官
    Office,
    /// 帝旺
    Peak,
    /// 衰
    Decline,
    /// 病
    Sick,
    /// 死
    Death,
    /// 墓
    Tomb,
    /// 绝
    End,
    /// 胎
    Fetus,
    /// 养
    Raise,
}

impl std::fmt::Display for TwelveStages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwelveStages::Birth => write!(f, "长生"),
            TwelveStages::Bath => write!(f, "沐浴"),
            TwelveStages::Crown => write!(f, "冠带"),
            TwelveStages::Office => write!(f, "临官"),
            TwelveStages::Peak => write!(f, "帝旺"),
            TwelveStages::Decline => write!(f, "衰"),
            TwelveStages::Sick => write!(f, "病"),
            TwelveStages::Death => write!(f, "死"),
            TwelveStages::Tomb => write!(f, "墓"),
            TwelveStages::End => write!(f, "绝"),
            TwelveStages::Fetus => write!(f, "胎"),
            TwelveStages::Raise => write!(f, "养"),
        }
    }
}
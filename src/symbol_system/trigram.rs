//! 八卦模块，定义八卦相关的基本类型和特征

use crate::basic::YinYang;

/// 八卦枚举，表示八种基本卦象
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigram {
    /// 乾（天）
    Qian,
    /// 兑（泽）
    Dui,
    /// 离（火）
    Li,
    /// 震（雷）
    Zhen,
    /// 巽（风）
    Xun,
    /// 坎（水）
    Kan,
    /// 艮（山）
    Gen,
    /// 坤（地）
    Kun,
}

impl Trigram {
    /// 获取卦象的三爻，从下到上
    pub fn lines(&self) -> [YinYang; 3] {
        match self {
            Trigram::Qian => [YinYang::Yang, YinYang::Yang, YinYang::Yang],
            Trigram::Dui => [YinYang::Yang, YinYang::Yang, YinYang::Yin],
            Trigram::Li => [YinYang::Yang, YinYang::Yin, YinYang::Yang],
            Trigram::Zhen => [YinYang::Yang, YinYang::Yin, YinYang::Yin],
            Trigram::Xun => [YinYang::Yin, YinYang::Yang, YinYang::Yang],
            Trigram::Kan => [YinYang::Yin, YinYang::Yang, YinYang::Yin],
            Trigram::Gen => [YinYang::Yin, YinYang::Yin, YinYang::Yang],
            Trigram::Kun => [YinYang::Yin, YinYang::Yin, YinYang::Yin],
        }
    }
}

impl std::fmt::Display for Trigram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trigram::Qian => write!(f, "乾"),
            Trigram::Dui => write!(f, "兑"),
            Trigram::Li => write!(f, "离"),
            Trigram::Zhen => write!(f, "震"),
            Trigram::Xun => write!(f, "巽"),
            Trigram::Kan => write!(f, "坎"),
            Trigram::Gen => write!(f, "艮"),
            Trigram::Kun => write!(f, "坤"),
        }
    }
}
//! 地支相关类型的模块

mod relationship;

/// 地支枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarthlyBranch {
    /// 子
    Zi,
    /// 丑
    Chou,
    /// 寅
    Yin,
    /// 卯
    Mao,
    /// 辰
    Chen,
    /// 巳
    Si,
    /// 午
    Wu,
    /// 未
    Wei,
    /// 申
    Shen,
    /// 酉
    You,
    /// 戌
    Xu,
    /// 亥
    Hai,
}
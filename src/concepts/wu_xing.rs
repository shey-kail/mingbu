use serde::Serialize;
use crate::concepts::gan_zhi::ChineseName;

/// 五行关系枚举
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum WuXingRelation {
    /// 相生
    Sheng,
    /// 相克
    Ke,
    /// 相刑
    Xing,
    /// 相害
    Hai,
    /// 相合
    He,
}

/// 五行枚举，表示事物的五行属性
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum WuXing {
    /// 木
    Wood,
    /// 火
    Fire,
    /// 土
    Earth,
    /// 金
    Metal,
    /// 水
    Water,
}

impl std::fmt::Display for WuXing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WuXing::Wood => write!(f, "木"),
            WuXing::Fire => write!(f, "火"),
            WuXing::Earth => write!(f, "土"),
            WuXing::Metal => write!(f, "金"),
            WuXing::Water => write!(f, "水"),
        }
    }
}

impl ChineseName for WuXing {
    fn chinese_name(&self) -> &'static str {
        match self {
            WuXing::Wood => "木",
            WuXing::Fire => "火",
            WuXing::Earth => "土",
            WuXing::Metal => "金",
            WuXing::Water => "水",
        }
    }
}
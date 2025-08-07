//! 五行模块，定义五行相关的基本类型和特征

mod relationship;

pub use relationship::WuXingRelation;

/// 五行枚举，表示事物的五行属性
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

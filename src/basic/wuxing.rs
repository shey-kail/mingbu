//! 五行模块，定义五行相关的基本类型和特征

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

impl WuXing {
    /// 获取相生的五行
    pub fn generating(&self) -> Self {
        match self {
            WuXing::Wood => WuXing::Fire,
            WuXing::Fire => WuXing::Earth,
            WuXing::Earth => WuXing::Metal,
            WuXing::Metal => WuXing::Water,
            WuXing::Water => WuXing::Wood,
        }
    }

    /// 获取相克的五行
    pub fn overcoming(&self) -> Self {
        match self {
            WuXing::Wood => WuXing::Earth,
            WuXing::Earth => WuXing::Water,
            WuXing::Water => WuXing::Fire,
            WuXing::Fire => WuXing::Metal,
            WuXing::Metal => WuXing::Wood,
        }
    }
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
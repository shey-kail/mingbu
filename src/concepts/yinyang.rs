use serde::Serialize;
use crate::concepts::gan_zhi::ChineseName;

/// 阴阳枚举
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum YinYang {
    /// 阳
    Yang,
    /// 阴
    Yin,
}

impl YinYang {
    /// 获取相反的阴阳属性
    pub fn opposite(&self) -> Self {
        match self {
            YinYang::Yang => YinYang::Yin,
            YinYang::Yin => YinYang::Yang,
        }
    }
}

impl ChineseName for YinYang {
    fn chinese_name(&self) -> &'static str {
        match self {
            YinYang::Yang => "阳",
            YinYang::Yin => "阴",
        }
    }
}
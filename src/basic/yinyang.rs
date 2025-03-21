//! 阴阳模块，定义阴阳相关的基本类型和特征

/// 阴阳枚举，表示事物的阴阳属性
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for YinYang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YinYang::Yang => write!(f, "阳"),
            YinYang::Yin => write!(f, "阴"),
        }
    }
}
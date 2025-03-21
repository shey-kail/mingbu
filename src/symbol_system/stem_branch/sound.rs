//! 纳音模块，定义五行纳音的基本类型和特征

use crate::basic::WuXing;

/// 纳音结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sound {
    /// 纳音名称
    name: &'static str,
    /// 五行属性
    wuxing: WuXing,
}

impl Sound {
    /// 创建一个新的纳音实例
    pub const fn new(name: &'static str, wuxing: WuXing) -> Self {
        Self { name, wuxing }
    }

    /// 获取纳音名称
    pub fn name(&self) -> &str {
        self.name
    }

    /// 获取纳音五行
    pub fn wuxing(&self) -> WuXing {
        self.wuxing
    }
}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.wuxing)
    }
}
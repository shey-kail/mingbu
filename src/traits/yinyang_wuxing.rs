//! 基础特性模块，定义阴阳和五行相关的特性

use crate::basic::{YinYang, WuXing};

/// 阴阳特性，定义获取和转换阴阳属性的方法
pub trait YinYangTrait {
    /// 获取阴阳属性
    fn yinyang(&self) -> YinYang;
}

/// 五行特性，定义获取和转换五行属性的方法
pub trait WuXingTrait {
    /// 获取五行属性
    fn wuxing(&self) -> WuXing;
}
use crate::concepts::{YinYang, WuXing};

/// 阴阳特征，定义获取和转换阴阳属性的方法
pub trait YinYangTrait {
    /// 获取阴阳属性
    fn yinyang(&self) -> YinYang;
}

/// 五行特征，定义获取和转换五行属性的方法
pub trait WuXingTrait {
    /// 获取五行属性
    fn wuxing(&self) -> WuXing;
}
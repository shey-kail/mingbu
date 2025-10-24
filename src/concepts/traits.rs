use crate::concepts::yinyang::YinYang;
use crate::concepts::wu_xing::WuXing;

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

/// 关系特征，用于描述事物间的双方关系
pub trait Relationship {
    type Item;
    /// 获取我对目标的关系（双方关系）
    fn relationship_with(&self, other: &Self) -> Vec<Self::Item>;
    /// 根据给定的关系反向推导出对应的目标
    fn from_relationship(&self, relationship: Self::Item) -> Option<Self> where Self: Sized;
}

/// 三方关系特征，用于描述事物间的三方关系
pub trait TripleRelationship {
    type Item;
    /// 获取与两个目标的关系（三方关系）
    fn relationship_with_triple(&self, second: &Self, third: &Self) -> Option<Self::Item>;
    fn from_relationship_triple(&self, relationship: Self::Item) -> (Option<Self>, Option<Self>) where Self: Sized;
}
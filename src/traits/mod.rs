//! 特征模块，定义命理学中常用的特征
pub mod yinyang_wuxing;
use yinyang_wuxing::*;
/// 中文名称特征，用于获取事物的中文名称
pub trait ChineseName {
    /// 获取中文名称
    fn chinese_name(&self) -> &'static str;
}

/// 索引特征，用于获取事物在序列中的索引，索引从1开始
pub trait Index {
    /// 从索引创建实例
    fn from_index(index: usize) -> Self;
    /// 获取索引值
    fn index(&self) -> usize;
}

/// 迭代特征，用于在相关事物间迭代
pub trait Iter {
    type Item;
    /// 获取下一个元素
    fn next(&self) -> Self::Item;
    /// 获取上一个元素
    fn prev(&self) -> Self::Item;
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
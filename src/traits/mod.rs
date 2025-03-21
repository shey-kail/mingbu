//! 特征模块，定义命理学中常用的特征

/// 中文名称特征，用于获取事物的中文名称
pub trait ChineseName {
    /// 获取中文名称
    fn chinese_name(&self) -> String;
}

/// 索引特征，用于获取事物在序列中的索引
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
    /// 获取与目标的关系（双方关系）
    fn relationship_with(&self, other: &Self) -> Self::Item;
}

/// 三方关系特征，用于描述事物间的三方关系
pub trait TripleRelationship {
    type Item;
    /// 获取与两个目标的关系（三方关系）
    fn relationship_with_triple(&self, second: &Self, third: &Self) -> Self::Item;
}
// src/concepts/shen_sha.rs - 神煞模块
// Placeholder for神煞相关实现
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ShenSha {
    QingLong,  // 青龙
    MingTang,  // 明堂
    TianXiang, // 天厢
    JuMen,     // 巨门
    // 可以添加更多的神煞
}
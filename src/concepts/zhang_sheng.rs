// src/concepts/zhang_sheng.rs - 长生十二神模块
// Placeholder for长生十二神相关实现
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ZhangSheng {
    ChangSheng, // 长生
    MuYu,       // 沐浴
    GuanDai,    // 冠带
    LinGuan,    // 临官
    DiWang,     // 帝旺
    Shuai,      // 衰
    Bing,       // 病
    Si,         // 死
    Mu,         // 暮
    Jue,        // 绝
    Tai,        // 胎
    Yang,       // 养
}
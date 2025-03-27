//! 公历模块，定义公历相关的基本类型和特征

use std::fmt;
use crate::time::date::DateTime;
use crate::time::lunar::LunarDate;

/// 公历日期结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarDate {
    /// 公历年
    pub year: i32,
    /// 公历月（1-12）
    pub month: u8,
    /// 公历日（1-31）
    pub day: u8,
    /// 小时（0-23）
    pub hour: u8,
    /// 分钟（0-59）
    pub minute: u8,
}

impl SolarDate {
    /// 创建一个新的公历日期
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> Self {
        // 这里可以添加参数验证逻辑
        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }

    /// 从DateTime转换为SolarDate
    pub fn from_date_time(date_time: &DateTime) -> Self {
        Self {
            year: date_time.year,
            month: date_time.month,
            day: date_time.day,
            hour: date_time.hour,
            minute: date_time.minute,
        }
    }

    /// 转换为DateTime
    pub fn to_date_time(&self) -> DateTime {
        DateTime {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
        }
    }

    /// 转换为农历日期（未实现）
    pub fn to_lunar(&self) -> LunarDate {
        // 实际实现中需要进行公历到农历的转换算法
        // 这里暂时返回一个固定值
        LunarDate {
            year: self.year,
            month: 1,
            day: 1,
            hour: 0,
            is_leap_month: false,
        }
    }
}

impl fmt::Display for SolarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}", 
            self.year, self.month, self.day, self.hour, self.minute)
    }
}
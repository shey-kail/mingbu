//! 封装tyme4rs库的日期时间模块
mod lunar;
mod solar;
mod consts;

use tyme4rs::tyme::lunar::LunarHour;
use tyme4rs::tyme::solar::SolarTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime {
    solar: Solar,
    lunar: Lunar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solar {
    time: SolarTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lunar {
    time: LunarHour,
}
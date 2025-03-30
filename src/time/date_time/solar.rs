// 封装的公历时间类型
use tyme4rs::tyme::{solar::SolarTime, Culture};
use super::consts::SOLAR_TERMS;
use super::super::date_time::Lunar;
use std::ops::Sub;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solar {
    pub time: SolarTime,
}
impl Solar {
    // 从年月日时分创建实例
    pub fn new(
        year: isize, month: usize, day: usize,
        hour: usize, min: usize
    ) -> Self {
        let solar = SolarTime::from_ymd_hms(
            year, month, day, hour, min, 0
        );
        Self { time: solar } 
    }
    // 获取详细信息，返回元组 (年, 月, 日, 时, 分)
    pub fn detail(&self) -> (isize, usize, usize, usize, usize) {
        let year = self.time.get_year();
        let month = self.time.get_month();
        let day = self.time.get_day();
        let hour = self.time.get_hour();
        let min = self.time.get_minute();
        (year, month, day, hour, min)
    }
    // 是否是闰年
    pub fn is_leap(&self) -> bool {
        self.time
            .get_solar_day()
            .get_solar_month()
            .get_solar_year()
            .is_leap()
    }
    // 获取节气及节气天数
    pub fn get_term(&self) -> (&str, usize) {
        let result_term = self.time
                                        .get_term()
                                        .get_name();
        let index = self.time
                                .get_solar_day()
                                .get_term_day()
                                .get_day_index();
        let term = SOLAR_TERMS
                                .iter()
                                .find(|&&term| term == result_term)
                                .unwrap();
        (term, index)
    }
    // 转为农历
    pub fn to_lunar(&self) -> Lunar {
        let lunar = self.time.get_lunar_hour();
        Lunar { time: lunar }
    }
}

impl Sub for Solar {
    type Output = BigInt;
    // 计算两个时间的时间差，返回秒数
    fn sub(&self, target: Self) -> Self::Output {
        let mut days: BigInt = self.day.subtract(target.get_solar_day());
        let cs: BigInt = self.hour * 3600 + self.minute * 60 + self.second;
        let ts: BigInt = target.get_hour() * 3600 + target.get_minute() * 60 + target.get_second();
        let mut seconds: BigInt = cs - ts;
        if seconds < 0 {
          seconds += 86400;
          days -= 1;
        }
        seconds += days * 86400;
        return seconds;
    }
}

// 未来大于过去
impl PartialOrd for Solar {
    fn partial_cmp(&self, target: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(target))
    }
}

impl Ord for Solar {
    fn cmp(&self, target: &Self) -> std::cmp::Ordering {
        let isbefore = self.time.is_before(target.time);
        let isafter = self.time.is_after(target.time);
        match (isbefore, isafter) {
            // 如果此时早于彼时，则返回小于
            (true, false) => std::cmp::Ordering::Less,
            // 如果此时晚于彼时，则返回大于
            (false, true) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

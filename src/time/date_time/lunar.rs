// 封装的农历时间类型
use tyme4rs::tyme::lunar::LunarHour;
use super::super::date_time::Lunar;

impl Lunar  {
    pub fn new(
        year: isize, month: isize, day: usize,
        hour: usize, min: usize
    ) -> Self {
        let solar = LunarHour::from_ymd_hms(
            year, month, day, hour, min, 0
        );
        Self { time: solar } 
    }
    pub fn detail(&self) -> (isize, isize, usize, usize) {
        let year = self.time.get_year();
        let month = self.time.get_month();
        let day = self.time.get_day();
        let hour = self.time.get_hour();
        (year, month, day, hour)
    }
    // 月份列表
    pub fn get_month_list(&self) -> Vec<String> {
        let mut month_list = Vec::new();
        for _ in 1..=12 {
            month_list.push(self.time.get_month().to_string());
        }
        month_list
    }
    // 转为公历
    // 转为节气
    // 总天数

}
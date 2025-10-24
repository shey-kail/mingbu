use crate::concepts::gan_zhi::{GanZhi, WuXingTrait};
use crate::concepts::wu_xing::WuXing;
use crate::calendar::chinese::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct BaZi {
    pub year: GanZhi,
    pub month: GanZhi,
    pub day: GanZhi,
    pub hour: GanZhi,
    pub day_master: WuXing,
}

impl BaZi {
    pub fn from_solar_date(year: i32, month: u32, day: u32, hour: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // 使用calendar模块从公历日期获取农历干支
        let datetime = DateTime::from_solar_date(year, month, day, hour)?;
        let (year_gan_zhi, month_gan_zhi, day_gan_zhi, hour_gan_zhi) = datetime.get_gan_zhi();
        
        // 从日柱的天干确定日干五行
        let day_master = day_gan_zhi.stem().wuxing();
        
        Ok(BaZi {
            year: year_gan_zhi,
            month: month_gan_zhi,
            day: day_gan_zhi,
            hour: hour_gan_zhi,
            day_master,
        })
    }
    
    pub fn from_gan_zhi(year: GanZhi, month: GanZhi, day: GanZhi, hour: GanZhi) -> Self {
        // 从日柱的天干确定日干五行
        let day_master = day.stem().wuxing();
        
        BaZi {
            year,
            month,
            day,
            hour,
            day_master,
        }
    }
}
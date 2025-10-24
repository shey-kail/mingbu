use serde::Serialize;
use crate::concepts::gan_zhi::GanZhi;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DateTime {
    year_gan_zhi: GanZhi,
    month_gan_zhi: GanZhi,
    day_gan_zhi: GanZhi,
    hour_gan_zhi: GanZhi,
}

impl DateTime {
    pub fn from_solar_date(year: i32, month: u32, day: u32, hour: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: 实现从公历到干支的转换算法
        // 目前返回占位符值
        let year_gan_zhi = GanZhi::new(
            crate::concepts::gan_zhi::HeavenlyStem::Jia,
            crate::concepts::gan_zhi::EarthlyBranch::Zi
        ).unwrap();
        let month_gan_zhi = GanZhi::new(
            crate::concepts::gan_zhi::HeavenlyStem::Jia,
            crate::concepts::gan_zhi::EarthlyBranch::Zi
        ).unwrap();
        let day_gan_zhi = GanZhi::new(
            crate::concepts::gan_zhi::HeavenlyStem::Jia,
            crate::concepts::gan_zhi::EarthlyBranch::Zi
        ).unwrap();
        let hour_gan_zhi = GanZhi::new(
            crate::concepts::gan_zhi::HeavenlyStem::Jia,
            crate::concepts::gan_zhi::EarthlyBranch::Zi
        ).unwrap();
        
        Ok(DateTime {
            year_gan_zhi,
            month_gan_zhi,
            day_gan_zhi,
            hour_gan_zhi,
        })
    }
    
    pub fn get_gan_zhi(&self) -> (GanZhi, GanZhi, GanZhi, GanZhi) {
        (
            self.year_gan_zhi,
            self.month_gan_zhi,
            self.day_gan_zhi,
            self.hour_gan_zhi
        )
    }
}
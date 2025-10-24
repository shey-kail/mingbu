pub mod ba_zi;
pub mod qi_zheng;

use crate::json;

pub fn ba_zi_json(year: i32, month: u32, day: u32, hour: u32) -> Result<String, json::MingbuError> {
    let bazi = ba_zi::BaZi::from_solar_date(year, month, day, hour)
        .map_err(|e| json::MingbuError {
            code: "BAZI_ERROR",
            message: e.to_string(),
        })?;
    json::to_json(&bazi)
}

pub fn qi_zheng_json(julian_day: f64) -> Result<String, json::MingbuError> {
    let pan = qi_zheng::QiZhengPan::from_julian_day(julian_day);
    json::to_json(&pan)
}
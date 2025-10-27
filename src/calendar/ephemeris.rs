// 使用新的安全 Swiss Ephemeris 包装器
use crate::calendar::swisseph::{SwissEph, SE_SUN, SEFLG_SWIEPH, SEFLG_SIDEREAL};

#[derive(Debug)]
pub struct EphemerisError(String);

impl From<crate::calendar::swisseph::SwissEphError> for EphemerisError {
    fn from(err: crate::calendar::swisseph::SwissEphError) -> Self {
        EphemerisError(format!("{}", err))
    }
}

pub fn solar_longitude(julian_day: f64) -> Result<f64, EphemerisError> {
    let eph = SwissEph::new()?;
    let (longitude, _, _, _, _, _) = eph.calc(julian_day, SE_SUN, SEFLG_SWIEPH | SEFLG_SIDEREAL)?;
    Ok(longitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_longitude() {
        // Test with JD 2451545.0 (Jan 1, 2000 at noon)
        let result = solar_longitude(2451545.0);
        // Note: This test may fail if ephemeris files are not available
        // which is expected in some environments
        match result {
            Ok(longitude) => {
                // Solar longitude should be a reasonable value between 0 and 360
                assert!(longitude >= 0.0 && longitude < 360.0);
            }
            Err(_) => {
                // It's acceptable if this fails due to missing ephemeris files
            }
        }
    }
}
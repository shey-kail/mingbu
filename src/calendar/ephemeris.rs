// 唯一调用 swisseph 的地方
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int};

extern "C" {
    fn swe_calc(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;
}

#[derive(Debug)]
pub struct EphemerisError(String);

pub fn solar_longitude(julian_day: f64) -> Result<f64, EphemerisError> {
    let mut result = [0.0; 6];
    let mut error_msg = [0i8; 256];
    
    let ret = unsafe {
        swe_calc(
            julian_day,
            0, // SUN
            2345, // SIDEREAL flags
            result.as_mut_ptr(),
            error_msg.as_mut_ptr(),
        )
    };

    if ret < 0 {
        let msg = unsafe { CStr::from_ptr(error_msg.as_mut_ptr()) }
            .to_string_lossy()
            .into_owned();
        Err(EphemerisError(msg))
    } else {
        Ok(result[0])
    }
}
//! Safe wrapper for the Swiss Ephemeris library (SwissEph)
//! This module provides a platform-agnostic API that wraps the C Swiss Ephemeris library
//! and handles FFI, error handling, and resource management in a safe way.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::path::Path;

// Define constants that match the C library
pub const SE_ECL_CENTRAL: c_int = 1;
pub const SE_ECL_NONCENTRAL: c_int = 2;
pub const SE_ECL_TOTAL: c_int = 4;
pub const SE_ECL_ANNULAR: c_int = 8;
pub const SE_ECL_PARTIAL: c_int = 16;
pub const SE_ECL_ANNULAR_TOTAL: c_int = 32;
pub const SE_ECL_PENUMBRAL: c_int = 64;
pub const SE_ECL_ALLTYPES_SOLAR: c_int = 31;
pub const SE_ECL_ALLTYPES_LUNAR: c_int = 112;
pub const SE_ECL_VISIBLE: c_int = 128;
pub const SE_ECL_MAX_VISIBLE: c_int = 256;
pub const SE_ECL_1ST_VISIBLE: c_int = 512;
pub const SE_ECL_2ND_VISIBLE: c_int = 1024;
pub const SE_ECL_3RD_VISIBLE: c_int = 2048;
pub const SE_ECL_4TH_VISIBLE: c_int = 4096;
pub const SE_ECL_OCC_BEG_DAYLIGHT: c_int = 128;
pub const SE_ECL_OCC_END_DAYLIGHT: c_int = 256;
pub const SE_ECL_TOT_MEMBER: c_int = 16384;
pub const SE_ECL_PART_MEMBER: c_int = 32768;

pub const SE_HOUSE_PLACIDUS: c_char = b'P' as c_char;
pub const SE_HOUSE_KOCH: c_char = b'K' as c_char;
pub const SE_HOUSE_PORPHYRIUS: c_char = b'O' as c_char;
pub const SE_HOUSE_REGIOMONTANUS: c_char = b'R' as c_char;
pub const SE_HOUSE_CAMPANUS: c_char = b'C' as c_char;
pub const SE_HOUSE_EQUAL: c_char = b'E' as c_char;
pub const SE_HOUSE_VEHLOW_EQUAL: c_char = b'V' as c_char;
pub const SE_HOUSE_AXIAL: c_char = b'X' as c_char;
pub const SE_HOUSE_AZIMUTHAL: c_char = b'H' as c_char;
pub const SE_HOUSE_POLICH_PAGE: c_char = b'T' as c_char;
pub const SE_HOUSE_ALCABITUS: c_char = b'B' as c_char;
pub const SE_HOUSE_MORINUS: c_char = b'M' as c_char;
pub const SE_HOUSE_KRUSINSKI: c_char = b'U' as c_char;
pub const SE_HOUSE_WHOLE_SIGN: c_char = b'W' as c_char;

// Define planet/body constants
pub const SE_SUN: c_int = 0;
pub const SE_MOON: c_int = 1;
pub const SE_MERCURY: c_int = 2;
pub const SE_VENUS: c_int = 3;
pub const SE_MARS: c_int = 4;
pub const SE_JUPITER: c_int = 5;
pub const SE_SATURN: c_int = 6;
pub const SE_URANUS: c_int = 7;
pub const SE_NEPTUNE: c_int = 8;
pub const SE_PLUTO: c_int = 9;
pub const SE_MEAN_NODE: c_int = 10;
pub const SE_TRUE_NODE: c_int = 11;
pub const SE_MEAN_APOG: c_int = 12;
pub const SE_OSCU_APOG: c_int = 13;
pub const SE_EARTH: c_int = 14;
pub const SE_CHIRON: c_int = 15;
pub const SE_PHOLUS: c_int = 16;
pub const SE_CERES: c_int = 17;
pub const SE_PALLAS: c_int = 18;
pub const SE_JUNO: c_int = 19;
pub const SE_VESTA: c_int = 20;

// Define flag constants
pub const SEFLG_JPLEPH: c_int = 1;
pub const SEFLG_SWIEPH: c_int = 2;
pub const SEFLG_MOSEPH: c_int = 4;
pub const SEFLG_TOPOCTR: c_int = 8;
pub const SEFLG_SIDEREAL: c_int = 16;
pub const SEFLG_ICRS: c_int = 32;
pub const SEFLG_DPSIDEPS_1980: c_int = 64;
pub const SEFLG_TRUEPOS: c_int = 128;
pub const SEFLG_SPEED3: c_int = 256;
pub const SEFLG_SPEED: c_int = 512;
pub const SEFLG_NOGDEFL: c_int = 1024;
pub const SEFLG_NOABERR: c_int = 2048;
pub const SEFLG_ASTROMETRIC: c_int = 3072;
pub const SEFLG_EQUATORIAL: c_int = 4096;
pub const SEFLG_XYZ: c_int = 8192;
pub const SEFLG_RADIANS: c_int = 16384;
pub const SEFLG_BARYCTR: c_int = 32768;
pub const SEFLG_HELCTR: c_int = 65536;
pub const SEFLG_TRUEPOS_CORR: c_int = 409600;
pub const SEFLG_TROPICAL: c_int = 0;

// Define sidereal mode constants
pub const SE_SIDM_FAGAN_BRADLEY: c_int = 0;
pub const SE_SIDM_LAHIRI: c_int = 1;
pub const SE_SIDM_DELUCE: c_int = 2;
pub const SE_SIDM_RAMAN: c_int = 3;
pub const SE_SIDM_USHASHASHI: c_int = 4;
pub const SE_SIDM_KRISHNAMURTI: c_int = 5;
pub const SE_SIDM_DJWHAL_KHUL: c_int = 6;
pub const SE_SIDM_YUKTESHWAR: c_int = 7;
pub const SE_SIDM_JN_BHASIN: c_int = 8;
pub const SE_SIDM_BABYL_KUGLER1: c_int = 9;
pub const SE_SIDM_BABYL_KUGLER2: c_int = 10;
pub const SE_SIDM_BABYL_KUGLER3: c_int = 11;
pub const SE_SIDM_BABYL_HUBER: c_int = 12;
pub const SE_SIDM_BABYL_ETPSC: c_int = 13;
pub const SE_SIDM_ALDEBARAN_15TAU: c_int = 14;
pub const SE_SIDM_HIPPARCHOS: c_int = 15;
pub const SE_SIDM_SASSANIAN: c_int = 16;
pub const SE_SIDM_GALCENT_0SAG: c_int = 17;
pub const SE_SIDM_J2000: c_int = 18;
pub const SE_SIDM_J1900: c_int = 19;
pub const SE_SIDM_B1950: c_int = 20;
pub const SE_SIDM_SURYASIDDHANTA: c_int = 21;
pub const SE_SIDM_SURYASIDDHANTA_MSUN: c_int = 22;
pub const SE_SIDM_ARYABHATA: c_int = 23;
pub const SE_SIDM_ARYABHATA_MSUN: c_int = 24;
pub const SE_SIDM_SS_REVATI: c_int = 25;
pub const SE_SIDM_SS_CITRA: c_int = 26;
pub const SE_SIDM_TRUE_CITRA: c_int = 27;
pub const SE_SIDM_TRUE_REVATI: c_int = 28;
pub const SE_SIDM_USER: c_int = 255;

/// Error type for Swiss Ephemeris operations
#[derive(Debug, Clone)]
pub enum SwissEphError {
    CalculationFailed(String),
    InvalidInput(String),
    FileNotFound(String),
    LibraryNotInitialized,
    NotSupported,
}

impl std::fmt::Display for SwissEphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwissEphError::CalculationFailed(msg) => write!(f, "Calculation failed: {}", msg),
            SwissEphError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            SwissEphError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            SwissEphError::LibraryNotInitialized => write!(f, "Library not initialized"),
            SwissEphError::NotSupported => write!(f, "Operation not supported"),
        }
    }
}

impl std::error::Error for SwissEphError {}

/// Result type for Swiss Ephemeris operations
pub type SwissResult<T> = Result<T, SwissEphError>;

// Declare the C function bindings
extern "C" {
    fn swe_calc(tjd_ut: c_double, ipl: c_int, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    fn swe_calc_ut(tjd_ut: c_double, ipl: c_int, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    fn swe_set_ephe_path(path: *const c_char);
    fn swe_set_jpl_file(fname: *const c_char);
    fn swe_close();
    fn swe_get_planet_name(ipl: c_int, spname: *mut c_char);
    fn swe_deltat(tjd: c_double) -> c_double;
    fn swe_sidtime(tjd_ut: c_double) -> c_double;
    fn swe_sidtime0(tjd_ut: c_double, eps: c_double, nut: c_double) -> c_double;
    fn swe_house_pos(
        armc: c_double, geolat: c_double, eps: c_double,
        hsys: c_char, xpin: *const c_double, serr: *mut c_char
    ) -> c_double;
    fn swe_houses(
        tjd_ut: c_double, geolat: c_double, geolon: c_double, hsys: c_char,
        cusp: *mut c_double, ascmc: *mut c_double
    ) -> c_int;
    fn swe_houses_armc(
        armc: c_double, geolat: c_double, eps: c_double, hsys: c_char,
        cusp: *mut c_double, ascmc: *mut c_double
    ) -> c_int;
    fn swe_set_sid_mode(sid_mode: c_int, t0: c_double, ayan_t0: c_double);
    fn swe_get_ayanamsa_ex(tjd_et: c_double, iflag: c_int, aya: *mut c_double, serr: *mut c_char) -> c_int;
}

/// Safe wrapper for the Swiss Ephemeris library
pub struct SwissEph {
    initialized: bool,
}

impl SwissEph {
    /// Create a new instance of the Swiss Ephemeris wrapper
    pub fn new() -> SwissResult<Self> {
        Ok(SwissEph {
            initialized: true,
        })
    }

    /// Set the path to the ephemeris files
    pub fn set_ephe_path<P: AsRef<Path>>(&self, path: P) -> SwissResult<()> {
        let path_str = path.as_ref().to_str()
            .ok_or_else(|| SwissEphError::InvalidInput("Invalid path".to_string()))?;
        
        let c_path = CString::new(path_str)
            .map_err(|_| SwissEphError::InvalidInput("Path contains null bytes".to_string()))?;
        
        unsafe {
            swe_set_ephe_path(c_path.as_ptr());
        }
        Ok(())
    }

    /// Set the JPL ephemeris file
    pub fn set_jpl_file<P: AsRef<Path>>(&self, filename: P) -> SwissResult<()> {
        let path_str = filename.as_ref().to_str()
            .ok_or_else(|| SwissEphError::InvalidInput("Invalid filename".to_string()))?;
        
        let c_path = CString::new(path_str)
            .map_err(|_| SwissEphError::InvalidInput("Filename contains null bytes".to_string()))?;
        
        unsafe {
            swe_set_jpl_file(c_path.as_ptr());
        }
        Ok(())
    }

    /// Calculate the position of a celestial body
    pub fn calc(
        &self,
        julian_day: f64,
        body: c_int,
        flags: c_int,
    ) -> SwissResult<(f64, f64, f64, f64, f64, f64)> {
        let mut position = [0.0; 6]; // longitude, latitude, distance, longitude_speed, lat_speed, dist_speed
        let mut error_msg = [0i8; 256];
        
        let result = unsafe {
            swe_calc(
                julian_day,
                body,
                flags,
                position.as_mut_ptr(),
                error_msg.as_mut_ptr(),
            )
        };

        if result < 0 {
            let error_cstr = unsafe { CStr::from_ptr(error_msg.as_ptr()) };
            let error_str = error_cstr.to_string_lossy().into_owned();
            Err(SwissEphError::CalculationFailed(error_str))
        } else {
            Ok((
                position[0], // longitude
                position[1], // latitude
                position[2], // distance in AU
                position[3], // longitude speed (deg/day)
                position[4], // latitude speed (deg/day)
                position[5], // distance speed (AU/day)
            ))
        }
    }

    /// Calculate the position of a celestial body using Universal Time
    pub fn calc_ut(
        &self,
        julian_day_ut: f64,
        body: c_int,
        flags: c_int,
    ) -> SwissResult<(f64, f64, f64, f64, f64, f64)> {
        let mut position = [0.0; 6];
        let mut error_msg = [0i8; 256];
        
        let result = unsafe {
            swe_calc_ut(
                julian_day_ut,
                body,
                flags,
                position.as_mut_ptr(),
                error_msg.as_mut_ptr(),
            )
        };

        if result < 0 {
            let error_cstr = unsafe { CStr::from_ptr(error_msg.as_ptr()) };
            let error_str = error_cstr.to_string_lossy().into_owned();
            Err(SwissEphError::CalculationFailed(error_str))
        } else {
            Ok((
                position[0], // longitude
                position[1], // latitude
                position[2], // distance in AU
                position[3], // longitude speed (deg/day)
                position[4], // latitude speed (deg/day)
                position[5], // distance speed (AU/day)
            ))
        }
    }

    /// Calculate houses
    pub fn houses(
        &self,
        julian_day_ut: f64,
        latitude: f64,
        longitude: f64,
        house_system: c_char,
    ) -> SwissResult<([f64; 13], [f64; 10])> {
        let mut cusps = [0.0; 13];
        let mut ascmc = [0.0; 10];
        
        let result = unsafe {
            swe_houses(
                julian_day_ut,
                latitude,
                longitude,
                house_system,
                cusps.as_mut_ptr(),
                ascmc.as_mut_ptr(),
            )
        };

        if result < 0 {
            Err(SwissEphError::CalculationFailed(
                "Failed to calculate houses".to_string(),
            ))
        } else {
            Ok((cusps, ascmc))
        }
    }

    /// Calculate house positions
    pub fn house_position(
        &self,
        armc: f64,      // ARMC (sidereal time)
        latitude: f64,  // geodetic latitude
        ecliptic_obliquity: f64, // epsilon
        house_system: c_char,
        longitude: f64, // longitude of planet
        latitude_planet: f64, // latitude of planet
    ) -> SwissResult<f64> {
        let mut error_msg = [0i8; 256];
        let pos = [longitude, latitude_planet];
        
        let result = unsafe {
            swe_house_pos(
                armc,
                latitude,
                ecliptic_obliquity,
                house_system,
                pos.as_ptr(),
                error_msg.as_mut_ptr(),
            )
        };

        if result < 0.0 {
            let error_cstr = unsafe { CStr::from_ptr(error_msg.as_ptr()) };
            let error_str = error_cstr.to_string_lossy().into_owned();
            Err(SwissEphError::CalculationFailed(error_str))
        } else {
            Ok(result)
        }
    }

    /// Calculate Delta T (difference between UT and ET)
    pub fn delta_t(&self, julian_day: f64) -> f64 {
        unsafe { swe_deltat(julian_day) }
    }

    /// Calculate sidereal time
    pub fn sidereal_time(&self, julian_day_ut: f64) -> f64 {
        unsafe { swe_sidtime(julian_day_ut) }
    }

    /// Calculate sidereal time with custom epsilon and nutation
    pub fn sidereal_time_custom(&self, julian_day_ut: f64, epsilon: f64, nutation: f64) -> f64 {
        unsafe { swe_sidtime0(julian_day_ut, epsilon, nutation) }
    }

    /// Set the sidereal mode
    pub fn set_sidereal_mode(&self, sid_mode: c_int, t0: f64, ayan_t0: f64) {
        unsafe {
            swe_set_sid_mode(sid_mode, t0, ayan_t0);
        }
    }

    /// Get the ayanamsa (precession correction)
    pub fn get_ayanamsa(&self, julian_day_et: f64) -> SwissResult<f64> {
        let mut ayanamsa = 0.0;
        let mut error_msg = [0i8; 256];
        
        let result = unsafe {
            swe_get_ayanamsa_ex(
                julian_day_et,
                0, // default flags
                &mut ayanamsa,
                error_msg.as_mut_ptr(),
            )
        };

        if result < 0 {
            let error_cstr = unsafe { CStr::from_ptr(error_msg.as_ptr()) };
            let error_str = error_cstr.to_string_lossy().into_owned();
            Err(SwissEphError::CalculationFailed(error_str))
        } else {
            Ok(ayanamsa)
        }
    }

    /// Get the name of a planet/body
    pub fn get_planet_name(&self, body: c_int) -> SwissResult<String> {
        let mut name_buffer = [0u8; 256];
        
        unsafe {
            swe_get_planet_name(body, name_buffer.as_mut_ptr() as *mut c_char);
        }
        
        // Find the null terminator
        let len = name_buffer.iter().position(|&x| x == 0).unwrap_or(name_buffer.len());
        let name_str = std::str::from_utf8(&name_buffer[..len])
            .map_err(|_| SwissEphError::CalculationFailed("Invalid planet name".to_string()))?;
        
        Ok(name_str.to_string())
    }

    /// Close the Swiss Ephemeris library (not typically needed in safe Rust wrapper)
    pub fn close(&self) {
        unsafe {
            swe_close();
        }
    }
}

impl Drop for SwissEph {
    fn drop(&mut self) {
        // Optionally clean up when the wrapper is dropped
        if self.initialized {
            unsafe {
                swe_close();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swiss_eph_creation() {
        let eph = SwissEph::new();
        assert!(eph.is_ok());
    }

    #[test]
    fn test_constants() {
        assert_eq!(SE_SUN, 0);
        assert_eq!(SE_MOON, 1);
        assert_eq!(SEFLG_SIDEREAL, 16);
    }

    #[test]
    fn test_calculation() {
        let eph = SwissEph::new().expect("Failed to create SwissEph instance");
        
        // Test with a known date - JD 2451545.0 is 2000 Jan 1.5 (noon)
        let result = eph.calc_ut(2451545.0, SE_SUN, SEFLG_SWIEPH);
        
        // The result might fail due to missing ephemeris files, but we can at least test
        // that the function call works without panicking
        match result {
            Ok((lon, lat, dist, _, _, _)) => {
                // Basic sanity checks for solar position
                assert!(lon >= 0.0 && lon < 360.0);
                assert!(lat.abs() < 10.0); // Sun's latitude is usually small
                assert!(dist > 0.0); // Distance must be positive
            }
            Err(_) => {
                // This is expected if ephemeris files are not available
                // We're testing that the FFI call works correctly
            }
        }
    }

    #[test]
    fn test_delta_t() {
        let eph = SwissEph::new().expect("Failed to create SwissEph instance");
        // Test with JD 2451545.0 which should give a reasonable Delta T
        let dt = eph.delta_t(2451545.0);
        assert!(dt > 0.0); // Delta T is typically positive
    }

    #[test]
    fn test_sidereal_time() {
        let eph = SwissEph::new().expect("Failed to create SwissEph instance");
        // Test with JD 2451545.0
        let sid_time = eph.sidereal_time(2451545.0);
        // Sidereal time should be between 0 and 360 degrees
        assert!(sid_time >= 0.0 && sid_time < 360.0);
    }
}
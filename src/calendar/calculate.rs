use std::collections::HashMap;
use crate::calendar::swisseph::{SwissEph, SE_SUN, SE_MOON, SE_MERCURY, SE_VENUS, SE_MARS, 
                               SE_JUPITER, SE_SATURN, SE_URANUS, SE_NEPTUNE, SE_PLUTO, SE_MEAN_NODE, 
                               SE_TRUE_NODE, SEFLG_SWIEPH, SEFLG_SIDEREAL};

// Constants from the Java class
pub struct Calculate {
    // Constants
    pub se_start: i32,
    pub se_fortune: i32,
    pub se_asc: i32,
    pub se_mc: i32,
    pub se_end: i32,
    pub speed_normal: i32,
    pub speed_reverse: i32,
    pub speed_eclipse: i32,
    pub speed_stationary: i32,
    pub speed_invisible: i32,
    pub speed_slow: i32,
    pub speed_fast: i32,
    pub transit_inc: f64,
    pub degree_precision: f64,
    pub half_degree_precision: f64,
    pub quarter_degree_precision: f64,
    pub max_speed: f64,
    pub minute: f64,
    pub half_minute: f64,
    pub invalid: f64,
    pub switch_year: i32,
    pub switch_month: i32,
    pub switch_day: i32,
    pub switch_ut: f64,
    pub beijing_time_offset: f64,

    // Instance fields
    min_ut: f64,
    max_ut: f64,
    one_hour: f64,
    reject_degree_precision: f64,
    to_sidereal_speed: f64,
    to_sidereal_offset: f64,
    jump_speed: f64,
    jump_period: f64,
    time_period: f64,
    time_precision: f64,
    period_range: f64,
    true_node_average_speed: f64,
    newton_degree_precision: f64,
    newton_max_iter: i32,
    sidereal_systems: Vec<i32>,
    ephe_flag: i32,
    house_system_index: i32,
    julian_day_ut: f64,
    julian_day: f64,
    mountain_offset: f64,
    computed: bool,
    leap_month: bool,
    day_fortune_mode: bool,
    sidereal_mode: bool,
    sun_pos: f64,
    moon_pos: f64,
    stationary_gap: Vec<f64>,
    invisible_gap: Vec<f64>,
    slow_speed: Vec<f64>,
    fast_speed: Vec<f64>,
    location: [f64; 3],
    computation: [f64; 6],
    ascmc: [f64; 10],
    azimuth: [f64; 3],
    pheno: [f64; 20], // required size
    orbit_data: [f64; 4],
    temp_cusp: [f64; 13],
    zodiac: Vec<String>,
    full_zodiac: Vec<String>,
    zodiac_name: Vec<String>,
    speed_state: Vec<String>,
    mountain_name: Vec<String>,
    house_system_char: Vec<String>,
    correction_key: String,
    equatorial_orbit: bool,
    orbit_body: i32,
    load_table: HashMap<String, String>,
    // Swiss Ephemeris instance
    eph: Option<SwissEph>,
}

impl Calculate {
    pub fn new() -> Self {
        Calculate {
            se_start: 1000,
            se_fortune: 1000,
            se_asc: 1001,
            se_mc: 1002,
            se_end: 1002,
            speed_normal: 0,
            speed_reverse: 1,
            speed_eclipse: 2,
            speed_stationary: 3,
            speed_invisible: 4,
            speed_slow: 5,
            speed_fast: 6,
            transit_inc: 1.0,
            degree_precision: 1.0,
            half_degree_precision: 0.5,
            quarter_degree_precision: 0.25,
            max_speed: 1.0,
            minute: 1.0 / (24.0 * 60.0),
            half_minute: 0.5 * (1.0 / (24.0 * 60.0)),
            invalid: f64::MIN,
            switch_year: 1582,
            switch_month: 10,
            switch_day: 15,
            switch_ut: 2299160.5,
            beijing_time_offset: 8.0 / 24.0,
            min_ut: -247640.0,
            max_ut: 3690082.0,
            one_hour: 1.0 / 24.0,
            reject_degree_precision: 1.0,
            to_sidereal_speed: 360.0 / 360.98564736629,
            to_sidereal_offset: 1.0 / 360.98564736629,
            jump_speed: 1.2,
            jump_period: 30.0,
            time_period: 1.05,
            time_precision: 1.0 / (24.0 * 60.0 * 1.05),
            period_range: 0.5,
            true_node_average_speed: -0.05299,
            newton_degree_precision: 0.01,
            newton_max_iter: 100,
            sidereal_systems: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 3, 10, 11, 12, 13, 14, 15, 16], // Using placeholders for SE_SIDM constants
            ephe_flag: 256, // Placeholder for SEFLG_SWIEPH
            house_system_index: 0,
            julian_day_ut: f64::MIN,
            julian_day: f64::MIN,
            mountain_offset: 0.0,
            computed: false,
            leap_month: false,
            day_fortune_mode: false,
            sidereal_mode: false,
            sun_pos: 0.0,
            moon_pos: 0.0,
            stationary_gap: vec![],
            invisible_gap: vec![],
            slow_speed: vec![],
            fast_speed: vec![],
            location: [0.0; 3],
            computation: [0.0; 6],
            ascmc: [0.0; 10],
            azimuth: [0.0; 3],
            pheno: [0.0; 20],
            orbit_data: [0.0; 4],
            temp_cusp: [0.0; 13],
            zodiac: vec![],
            full_zodiac: vec![],
            zodiac_name: vec![],
            speed_state: vec![],
            mountain_name: vec![],
            house_system_char: vec![],
            correction_key: String::new(),
            equatorial_orbit: false,
            orbit_body: 0,
            load_table: HashMap::new(),
            eph: SwissEph::new().ok(), // Initialize Swiss Ephemeris, or None if it fails
        }
    }

    pub fn load_resource(&mut self) {
        // This method would load resource strings, but we're mocking them for now
        // In a real implementation, you would load from actual resource files
        self.zodiac = vec!["Aries".to_string(), "Taurus".to_string(), "Gemini".to_string(),
                           "Cancer".to_string(), "Leo".to_string(), "Virgo".to_string(),
                           "Libra".to_string(), "Scorpio".to_string(), "Sagittarius".to_string(),
                           "Capricorn".to_string(), "Aquarius".to_string(), "Pisces".to_string()];
        self.full_zodiac = self.zodiac.clone(); // Just using same values for now
        self.speed_state = vec!["Normal".to_string(), "Reverse".to_string(), "Eclipse".to_string(),
                                "Stationary".to_string(), "Invisible".to_string(), "Slow".to_string(),
                                "Fast".to_string()];
        self.house_system_char = vec!["P".to_string(), "K".to_string()]; // Just using placeholders
        self.zodiac_name = vec!["AR".to_string(), "TA".to_string(), "GE".to_string(),
                                "CA".to_string(), "LE".to_string(), "VI".to_string(),
                                "LI".to_string(), "SC".to_string(), "SA".to_string(),
                                "CP".to_string(), "AQ".to_string(), "PI".to_string()];
        self.mountain_name = vec!["1".to_string(), "2".to_string(), "3".to_string(),
                                  "4".to_string(), "5".to_string(), "6".to_string(),
                                  "7".to_string(), "8".to_string(), "9".to_string(),
                                  "10".to_string(), "11".to_string(), "12".to_string(),
                                  "13".to_string(), "14".to_string(), "15".to_string(),
                                  "16".to_string(), "17".to_string(), "18".to_string(),
                                  "19".to_string(), "20".to_string(), "21".to_string(),
                                  "22".to_string(), "23".to_string(), "24".to_string()];

        self.stationary_gap = vec![0.05, 0.1, 0.15]; // Sample values
        self.invisible_gap = vec![5.0, 10.0, 15.0]; // Sample values
        self.slow_speed = vec![0.1, 0.2, 0.3]; // Sample values
        self.fast_speed = vec![1.1, 1.2, 1.3]; // Sample values
        self.correction_key = "fixstar_equ_adjustments".to_string();

        self.set_topocentric_mode(false, false);
    }

    pub fn set_eph_mode(&mut self, use_moseph: bool) {
        // In a real implementation, this would handle ephemeris mode
        // For now, just updating the flag
        if use_moseph {
            self.ephe_flag = 4; // SEFLG_MOSEPH = 4
        } else {
            self.ephe_flag = 2; // SEFLG_SWIEPH = 2
        }
    }

    pub fn get_eph_mode(&self) -> bool {
        self.ephe_flag == 4 // SEFLG_MOSEPH = 4
    }

    pub fn set_topocentric_mode(&mut self, override_val: bool, val: bool) {
        // In a real implementation, this would handle topocentric mode
        // For now, just updating the flag
        if override_val && val {
            self.ephe_flag |= 8; // SEFLG_TOPOCTR = 8
            // location[2] would be set to altitude, but we're not implementing that for now
        } else {
            self.ephe_flag &= !8; // SEFLG_TOPOCTR = 8
            self.location[2] = 0.0;
        }
    }

    pub fn set_julian_day(&mut self, date: &[i32]) -> bool {
        let success = self.set_julian_day_value(self.get_julian_day_ut(date));
        if self.julian_day_ut < self.min_ut || self.julian_day_ut > self.max_ut {
            return false;
        }
        success
    }

    pub fn set_julian_day_value(&mut self, jd_ut: f64) -> bool {
        self.computed = false;
        self.julian_day_ut = jd_ut;
        if jd_ut != self.invalid {
            // For now, using a simple delta T approximation
            let delta_t = self.get_delta_t(jd_ut);
            self.julian_day = jd_ut + delta_t;
            true
        } else {
            self.julian_day = self.invalid;
            false
        }
    }

    fn get_delta_t(&self, ut: f64) -> f64 {
        // Use Swiss Ephemeris for Delta T calculation if available
        match &self.eph {
            Some(eph) => eph.delta_t(ut),
            None => {
                // Fallback to simplified calculation if Swiss Ephemeris is not available
                let year = self.get_year_from_julian_day(ut);
                if year < 948.0 {
                    let u = (year - 2000.0) / 100.0;
                    10.0 * u * u + 32.0 * u + 62.0
                } else {
                    let u = (year - 1820.0) / 100.0;
                    -20.0 + 32.0 * u - 0.5628 * u * u
                }
            }
        }
    }

    fn get_year_from_julian_day(&self, jd: f64) -> f64 {
        // This is a simplified approximation
        // In real implementation, proper astronomical algorithms would be used
        let mut a = jd + 0.5;
        a = a.floor();
        let b = a + 1524.0;
        let c = (b - 122.1) / 365.25;
        let d = 365.25 * c.floor();
        let e = (b - d) / 30.6001;
        let month = e.floor();
        let day = b - d - 30.6001 * e;
        let year = if month < 14.0 { c - 4716.0 } else { c - 4715.0 };
        year
    }

    pub fn get_julian_day_ut(&self, date: &[i32]) -> f64 {
        let d_hour = date[3] as f64 + date[4] as f64 / 60.0;
        let cal_type = if date[0] < self.switch_year || 
                      (date[0] == self.switch_year && 
                       (date[1] < self.switch_month || 
                        (date[1] == self.switch_month && date[2] < self.switch_day))) {
            // Julian calendar
            self.get_julian_day(date[0], date[1], date[2], d_hour, true)
        } else {
            // Gregorian calendar
            self.get_julian_day(date[0], date[1], date[2], d_hour, false)
        };
        cal_type
    }

    fn get_julian_day(&self, year: i32, month: i32, day: i32, hour: f64, is_julian: bool) -> f64 {
        let mut a = (14 - month) / 12;
        let y = year - a;
        let m = month + 12 * a - 3;
        
        let julian_day = day + (153 * m + 2) / 5 + 365 * y + y / 4;
        let julian_day = julian_day - if is_julian { 0 } else { y / 100 - y / 400 - 2 };
        let julian_day = julian_day + 1721119; // Julian day at March 1, year 1
        
        julian_day as f64 + hour / 24.0 - 0.5 // Subtract 0.5 because JD starts at noon
    }

    pub fn get_julian_day_from_ut(ut: f64) -> f64 {
        let delta_t = 0.0; // Simplified
        ut + delta_t
    }

    pub fn get_julian_day_ut_from_date(&self, jd_ut: f64, date: &mut [i32; 5]) {
        let jd = jd_ut + 0.5;
        let z = jd.floor() as i32;
        let f = jd - z as f64;
        
        let mut a = z;
        if z >= 2299161 {
            let alpha = ((z - 1867216) as f64 - 0.25) / 36524.25;
            a = z + 1 + alpha.floor() as i32 - (alpha.floor() as f64 / 4.0).floor() as i32;
        }
        
        let b = a + 1524;
        let c = ((b as f64 - 122.1) / 365.25).floor() as i32;
        let d = (365.25 * c as f64).floor() as i32;
        let e = ((b - d) as f64 / 30.6001).floor() as i32;
        
        date[2] = b - d - (30.6001 * e as f64).floor() as i32;
        date[1] = if e < 14 { e - 1 } else { e - 13 };
        date[0] = if date[1] > 2 { c - 4716 } else { c - 4715 };
        
        let hour = (f + 0.5) * 24.0; // Add 0.5 for the time component
        date[3] = hour as i32;
        date[4] = ((hour - date[3] as f64) * 60.0) as i32;
    }

    pub fn get_julian_day_ut_value(&self) -> f64 {
        self.julian_day_ut
    }

    pub fn get_julian_day_value(&self) -> f64 {
        self.julian_day
    }

    pub fn set_location(&mut self, longitude: f64, latitude: f64) {
        self.location[0] = longitude;
        self.location[1] = latitude;
        // In real implementation, would call swe_set_topo
    }

    pub fn set_location_from_array(&mut self, loc: &[f64]) {
        if loc.len() >= 2 {
            self.location[0] = loc[0];
            self.location[1] = loc[1];
        }
    }

    pub fn get_location(&self, loc: &mut [f64]) {
        if loc.len() >= 2 {
            loc[0] = self.location[0];
            loc[1] = self.location[1];
        }
    }

    pub fn get_longitude(&self) -> f64 {
        self.location[0]
    }

    pub fn get_latitude(&self) -> f64 {
        self.location[1]
    }

    pub fn get_difference_in_days(&self, from_date: &[i32], to_date: &[i32]) -> i32 {
        let from_jd = self.get_julian_day_ut(from_date);
        let to_jd = self.get_julian_day_ut(to_date);
        (to_jd - from_jd).round() as i32
    }

    pub fn compute(&mut self, jd_ut: f64, body: i32) -> f64 {
        let ut_sav = self.julian_day_ut;
        self.julian_day_ut = jd_ut;
        let val = self.compute_single(body);
        self.julian_day_ut = ut_sav;
        val
    }

    pub fn compute_single(&mut self, body: i32) -> f64 {
        if body >= self.se_start && body <= self.se_end {
            return self.compute_special(body);
        } else if body < 0 {
            return self.compute_orbit();
        }
        
        // Map Java body constants to Swiss Ephemeris constants
        let se_body = match body {
            0 => SE_SUN,
            1 => SE_MOON,
            2 => SE_MERCURY,
            3 => SE_VENUS,
            4 => SE_MARS,
            5 => SE_JUPITER,
            6 => SE_SATURN,
            7 => SE_URANUS,
            8 => SE_NEPTUNE,
            9 => SE_PLUTO,
            10 => SE_MEAN_NODE, // North Node (Rahu)
            11 => SE_TRUE_NODE, // True Node
            _ => {
                // For other bodies, return a default value
                self.computation[0] = 0.0;
                self.computation[3] = 0.0;
                self.computed = true;
                return self.computation[0];
            }
        };
        
        // Determine the appropriate flags to use
        let flags = if self.sidereal_mode {
            SEFLG_SWIEPH | SEFLG_SIDEREAL
        } else {
            SEFLG_SWIEPH
        };
        
        // Use Swiss Ephemeris to calculate the actual position
        match &self.eph {
            Some(eph) => {
                match eph.calc(self.julian_day_ut, se_body, flags) {
                    Ok((longitude, latitude, distance, long_speed, lat_speed, dist_speed)) => {
                        self.computation[0] = longitude; // longitude
                        self.computation[1] = latitude;  // latitude
                        self.computation[2] = distance;  // distance
                        self.computation[3] = long_speed; // longitude speed
                        self.computation[4] = lat_speed;  // latitude speed
                        self.computation[5] = dist_speed; // distance speed
                        self.computed = true;
                        self.computation[0] // Return longitude
                    }
                    Err(_) => {
                        // If calculation fails, return invalid
                        self.computation[0] = self.invalid;
                        self.computed = false;
                        self.computation[0]
                    }
                }
            }
            None => {
                // If Swiss Ephemeris is not available, return a default value
                self.computation[0] = 0.0;
                self.computation[3] = 0.0;
                self.computed = true;
                self.computation[0]
            }
        }
    }

    fn compute_special(&mut self, body: i32) -> f64 {
        self.computation[0] = self.invalid;
        self.computation[1] = 0.0;
        self.computation[2] = 0.0;
        self.computation[3] = 0.0;
        
        match body {
            x if x == self.se_asc => {
                // Ascendant - this is calculated during house calculation
                self.computation[0] = self.ascmc[0];
            },
            x if x == self.se_mc => {
                // Midheaven - this is calculated during house calculation
                self.computation[0] = self.ascmc[1];
            },
            x if x == self.se_fortune => {
                // Fortune - calculate using Swiss Ephemeris positions
                if self.ascmc[0] != self.invalid {
                    // Calculate Sun and Moon positions to use in Fortune calculation
                    // If we have the positions already, use them, otherwise calculate
                    let sun_pos = if self.sun_pos != 0.0 && self.julian_day_ut != f64::MIN {
                        self.sun_pos
                    } else {
                        match &self.eph {
                            Some(eph) => {
                                let flags = if self.sidereal_mode {
                                    SEFLG_SWIEPH | SEFLG_SIDEREAL
                                } else {
                                    SEFLG_SWIEPH
                                };
                                match eph.calc(self.julian_day_ut, SE_SUN, flags) {
                                    Ok((longitude, _, _, _, _, _)) => longitude,
                                    Err(_) => 0.0,
                                }
                            },
                            None => 0.0,
                        }
                    };
                    
                    let moon_pos = if self.moon_pos != 0.0 && self.julian_day_ut != f64::MIN {
                        self.moon_pos
                    } else {
                        match &self.eph {
                            Some(eph) => {
                                let flags = if self.sidereal_mode {
                                    SEFLG_SWIEPH | SEFLG_SIDEREAL
                                } else {
                                    SEFLG_SWIEPH
                                };
                                match eph.calc(self.julian_day_ut, SE_MOON, flags) {
                                    Ok((longitude, _, _, _, _, _)) => longitude,
                                    Err(_) => 0.0,
                                }
                            },
                            None => 0.0,
                        }
                    };
                    
                    let mut gap = moon_pos - sun_pos;
                    if !self.day_fortune_mode {
                        gap = -gap;
                    }
                    self.computation[0] = Self::normalize_degree(self.ascmc[0] + gap);
                }
            },
            _ => {}
        }
        
        self.computation[0]
    }

    fn normalize_degree(degree: f64) -> f64 {
        let mut deg = degree;
        while deg < 0.0 {
            deg += 360.0;
        }
        while deg >= 360.0 {
            deg -= 360.0;
        }
        deg
    }

    fn compute_orbit(&mut self) -> f64 {
        let day = self.get_julian_day_value();
        if day != self.invalid {
            self.computed = true;
            let offset = self.orbit_data[0] * (day - self.orbit_data[1]);
            let mut degree = self.orbit_data[2] + offset;
            
            if self.sidereal_mode {
                // In real implementation, this would subtract ayanamsa
                degree -= 0.0; // Placeholder
            }
            
            self.computation[0] = Self::normalize_degree(degree);
        } else {
            self.computed = false;
            self.computation[0] = self.invalid;
        }
        
        self.computation[1] = self.orbit_data[3]; // latitude
        self.computation[2] = 1.0; // distance in AU
        self.computation[3] = self.orbit_data[0]; // speed in longitude (degree / day)
        
        self.computation[0] // longitude
    }

    fn get_speed(&self) -> f64 {
        if self.computed { self.computation[3] } else { 0.0 }
    }

    pub fn get_degree_gap(pos1: f64, pos2: f64) -> f64 {
        let gap = (pos1 - pos2).abs();
        if gap > 180.0 { 360.0 - gap } else { gap }
    }

    pub fn get_speed_state_name(&self, state: i32, blank: &str) -> String {
        if state == self.speed_normal { blank.to_string() } else { self.speed_state[state as usize].clone() }
    }

    pub fn get_speed_state_name_array(&self) -> &Vec<String> {
        &self.speed_state
    }

    pub fn set_house_system_index(&mut self, index: i32) {
        self.house_system_index = index;
    }

    pub fn compute_houses(&mut self, cusps: &mut [f64]) {
        self.compute_houses_from_jd(cusps, self.julian_day_ut);
    }

    pub fn compute_houses_from_jd(&mut self, cusps: &mut [f64], ut: f64) {
        // Determine house system based on house_system_index
        let house_system = match self.house_system_index {
            0 => b'P' as i8, // Placidus
            1 => b'K' as i8, // Koch
            2 => b'O' as i8, // Porphyrius
            3 => b'R' as i8, // Regiomontanus
            4 => b'C' as i8, // Campanus
            5 => b'E' as i8, // Equal
            6 => b'V' as i8, // Vehlow Equal
            7 => b'X' as i8, // Axial/Horizontal
            8 => b'H' as i8, // Azimuthal/Horizontal
            9 => b'T' as i8, // Polich/Page
            10 => b'B' as i8, // Alcabitius
            11 => b'M' as i8, // Morinus
            12 => b'U' as i8, // Krusinski-Pisa
            13 => b'W' as i8, // Whole Sign
            _ => b'P' as i8, // Default to Placidus
        } as i8;
        
        // Use Swiss Ephemeris to calculate houses if available
        match &self.eph {
            Some(eph) => {
                match eph.houses(ut, self.location[1], self.location[0], house_system as i8) {
                    Ok((computed_cusps, ascmc)) => {
                        // Copy calculated cusps to the output array
                        for i in 0..std::cmp::min(13, cusps.len()) {
                            cusps[i] = if i < 13 { computed_cusps[i] } else { 0.0 };
                        }
                        
                        // Set Ascendant and MC
                        self.ascmc[0] = ascmc[0]; // Ascendant
                        self.ascmc[1] = ascmc[1]; // MC
                        self.ascmc[2] = ascmc[2]; // ARMC
                        self.ascmc[3] = ascmc[3]; // Vertex
                        self.ascmc[4] = ascmc[4]; // Equatorial Ascendant
                        self.ascmc[5] = ascmc[5]; // Co-Ascendant (W. Koch)
                        self.ascmc[6] = ascmc[6]; // Co-Ascendant (M. Munkasey)
                        self.ascmc[7] = ascmc[7]; // Polar Ascendant
                        self.ascmc[8] = ascmc[8]; // Nadir
                        self.ascmc[9] = ascmc[9]; // Zenith
                    }
                    Err(_) => {
                        // Fallback to simplified calculation if Swiss Ephemeris fails
                        if cusps.len() >= 13 {
                            for i in 0..12 {
                                cusps[i + 1] = (i as f64) * 30.0; // Placeholder house positions
                            }
                        }
                        
                        // Set Ascendant and MC
                        if cusps.len() >= 13 {
                            self.ascmc[0] = cusps[1]; // Ascendant
                            self.ascmc[1] = cusps[10]; // MC
                        } else {
                            self.ascmc[0] = self.invalid;
                            self.ascmc[1] = self.invalid;
                        }
                    }
                }
            }
            None => {
                // Fallback to simplified calculation if Swiss Ephemeris is not available
                if cusps.len() >= 13 {
                    for i in 0..12 {
                        cusps[i + 1] = (i as f64) * 30.0; // Placeholder house positions
                    }
                }
                
                // Set Ascendant and MC
                if cusps.len() >= 13 {
                    self.ascmc[0] = cusps[1]; // Ascendant
                    self.ascmc[1] = cusps[10]; // MC
                } else {
                    self.ascmc[0] = self.invalid;
                    self.ascmc[1] = self.invalid;
                }
            }
        }
    }

    pub fn init_special(&mut self, sun_long: f64, moon_long: f64, day: bool) {
        // If positions are provided (not invalid), use them
        if sun_long != self.invalid {
            self.sun_pos = sun_long;
        } else {
            // Otherwise try to calculate them using Swiss Ephemeris
            match &self.eph {
                Some(eph) => {
                    let flags = if self.sidereal_mode {
                        SEFLG_SWIEPH | SEFLG_SIDEREAL
                    } else {
                        SEFLG_SWIEPH
                    };
                    if let Ok((longitude, _, _, _, _, _)) = eph.calc(self.julian_day_ut, SE_SUN, flags) {
                        self.sun_pos = longitude;
                    }
                },
                None => {
                    self.sun_pos = 0.0; // Default fallback
                }
            }
        }

        if moon_long != self.invalid {
            self.moon_pos = moon_long;
        } else {
            // Otherwise try to calculate them using Swiss Ephemeris
            match &self.eph {
                Some(eph) => {
                    let flags = if self.sidereal_mode {
                        SEFLG_SWIEPH | SEFLG_SIDEREAL
                    } else {
                        SEFLG_SWIEPH
                    };
                    if let Ok((longitude, _, _, _, _, _)) = eph.calc(self.julian_day_ut, SE_MOON, flags) {
                        self.moon_pos = longitude;
                    }
                },
                None => {
                    self.moon_pos = 0.0; // Default fallback
                }
            }
        }

        self.day_fortune_mode = day || false; // night_fortune_mode == 0
    }

    pub fn compute_aspects(&self, f_pos: &[f64], t_pos: &[f64], 
                          aspects_degree: &[f64], aspects_tolerance: &[f64]) -> Vec<Vec<i32>> {
        let mut aspects = vec![vec![0; t_pos.len()]; f_pos.len()];
        
        for i in 0..f_pos.len() {
            if f_pos[i] == self.invalid { continue; }
            for j in 0..t_pos.len() {
                if t_pos[j] == self.invalid || (i == j && std::ptr::eq(f_pos, t_pos)) { 
                    continue; 
                }
                let angle = Self::get_degree_gap(f_pos[i], t_pos[j]);
                for k in 0..aspects_tolerance.len() {
                    if (angle - aspects_degree[k]).abs() <= aspects_tolerance[k] {
                        aspects[i][j] = (k + 1) as i32;
                        break;
                    }
                }
            }
        }
        
        aspects
    }

    pub fn beijing_time(ut: f64) -> f64 {
        ut + 8.0 / 24.0
    }

    pub fn is_valid(val: f64) -> bool {
        val != f64::MIN
    }

    pub fn get_zodiac(&self, degree: f64, full: bool) -> String {
        let index = (degree / 30.0) as usize;
        if full && index < self.full_zodiac.len() {
            self.full_zodiac[index].clone()
        } else if index < self.zodiac.len() {
            self.zodiac[index].clone()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn get_mountain(&self, degree: f64) -> String {
        let mut val = degree - 22.5 + self.mountain_offset;
        if val < 0.0 {
            val += 360.0;
        }
        let index = (val / 15.0) as usize;
        let val = 15.0 + (index as f64) * 15.0 - val;
        if index < self.mountain_name.len() {
            self.mountain_name[index].clone()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn get_elemental_index(&self, degree: f64) -> i32 {
        ((degree / 30.0) as i32) % 4
    }

    pub fn get_elemental_state_index(&self, degree: f64) -> i32 {
        ((degree / 30.0) as i32) % 3
    }

    pub fn set_mountain_offset(&mut self, val: f64) {
        self.mountain_offset = val;
    }

    pub fn get_chinese_year(&self, year: i32) -> i32 {
        // For now, using a placeholder - would normally use Resource.getInt("birth_year_base")
        let base = 1900; // Placeholder
        let mut year_val = (year - base + 1) % 60;
        if year_val < 0 { year_val += 60; }
        if year_val == 0 { year_val = 60; }
        year_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_initialization() {
        let calc = Calculate::new();
        assert_eq!(calc.invalid, f64::MIN);
        assert_eq!(calc.speed_normal, 0);
    }

    #[test]
    fn test_julian_day_calculation() {
        let calc = Calculate::new();
        let date = [2000, 1, 1, 12, 0]; // Jan 1, 2000 at 12:00
        let jd = calc.get_julian_day_ut(&date);
        // The expected value for JD 2000 Jan 1.5 is 2451545.0
        assert!((jd - 2451545.0).abs() < 1.0); // Allow some tolerance
    }

    #[test]
    fn test_degree_normalization() {
        assert_eq!(Calculate::normalize_degree(370.0), 10.0);
        assert_eq!(Calculate::normalize_degree(-10.0), 350.0);
    }
}

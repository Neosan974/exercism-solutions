// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const SECONDS_IN_EARTH_YEAR: u64 = 31_557_600;

/// Duration in seconds
#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { 0: s }
    }
}

pub trait Planet {
    fn get_ratio_from_earth_year() -> f64;
    fn get_duration_of_year() -> Duration {
        Duration {
            0: (Self::get_ratio_from_earth_year() * SECONDS_IN_EARTH_YEAR as f64) as u64,
        }
    }
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64) / (Self::get_duration_of_year().0 as f64)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn get_ratio_from_earth_year() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn get_ratio_from_earth_year() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn get_ratio_from_earth_year() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn get_ratio_from_earth_year() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn get_ratio_from_earth_year() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn get_ratio_from_earth_year() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn get_ratio_from_earth_year() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn get_ratio_from_earth_year() -> f64 {
        164.79132
    }
}

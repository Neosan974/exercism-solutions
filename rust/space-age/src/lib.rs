// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use planet_macro::planet_ratio;

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
        ((d.0 as f64) / (Self::get_duration_of_year().0 as f64)) as f64
    }
}

#[planet_ratio(0.2408467)]
pub struct Mercury;
#[planet_ratio(0.61519726)]
pub struct Venus;
#[planet_ratio(1.0)]
pub struct Earth;
#[planet_ratio(1.8808158)]
pub struct Mars;
#[planet_ratio(11.862615)]
pub struct Jupiter;
#[planet_ratio(29.447498)]
pub struct Saturn;
#[planet_ratio(84.016846)]
pub struct Uranus;
#[planet_ratio(164.79132)]
pub struct Neptune;

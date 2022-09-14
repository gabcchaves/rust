// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed > 0 && speed < 5 {
        return (speed as f64) * 221.0;
    }
    if speed >= 5 && speed < 9 {
        return (speed as f64) * 221.0 * 0.9;
    }
    return (speed as f64) * 221.0 * 0.77;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32;
}

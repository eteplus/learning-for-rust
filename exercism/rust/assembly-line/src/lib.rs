// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let products = speed as f64 * 221.0;
    let result = match speed {
        1..=4 => products,
        5..=8 => products * 0.9,
        9..=10 => products * 0.77,
        _ => products,
    };
    result
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let result = production_rate_per_hour(speed) / 60.0;
    result.floor() as u32
}

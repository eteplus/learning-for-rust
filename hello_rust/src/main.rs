#![allow(dead_code)]
mod authentication;
mod libs;
use libs::*;

fn main() {
    hello_cargo::run();
    println!("2 + 3 = {}\n", basic_math::add(2, 3));
    // car::run();
    // car2::run();
    // car3::run();
    // car4::run();
    // fruit::run();
    // person::run();
    // file::run();
    // lifetime::run();
    // trait1::run();
    // trait2::run();
    // counter::run();
    // container::run();
    // groups::run();
    // auth::run();
    // regex1::run();
    text_processing::run();
}
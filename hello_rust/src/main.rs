#![allow(dead_code)]
mod authentication;
mod libs;
use libs::*;

pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return "".to_string();
    }
    let string = phrase
        .split(' ')
        .map(|word| word
            .matches(char::is_uppercase)
            .collect::<Vec<&str>>().join(""))
        .collect::<Vec<String>>();
    string.join("")
}

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
    abbreviate("HyperText Markup Language");
    text_processing::run();
}
// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in 0..option.unwrap() {
        res += x;
    }
    println!("{}", res);
}

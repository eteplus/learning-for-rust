pub fn run() {
    println!("\nFruit!");
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 1, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let num: Option<u8> = Some(8);
    if let Some(8) = num {
        println!("It's 8!");
    }

    let gift = Some("candy");
    assert_eq!(gift.unwrap_or("least"), "candy");

    let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
    assert_eq!(empty_gift.unwrap_or("candy"), "candy");
}

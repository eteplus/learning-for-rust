pub fn verse(n: u32) -> String {
    if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    if n == 1 {
        return format!("{num} bottle of beer on the wall, {num} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", num = n);
    }
    format!("{num} bottles of beer on the wall, {num} bottles of beer.\nTake one down and pass it around, {left} {bottle} of beer on the wall.\n", num = n, left = n-1, bottle = if n == 2 { "bottle" } else { "bottles" })
}

pub fn sing(start: u32, end: u32) -> String {
    let mut str = String::from("");
    for i in (end..=start).rev() {
        str.push_str(&verse(i));
        if i != end {
            str.push_str("\n");
        }
    }
    str
}

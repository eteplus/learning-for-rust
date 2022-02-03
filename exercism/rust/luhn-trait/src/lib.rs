pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

fn is_valid(code: String) -> bool {
    let mut digits = code.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
    digits.reverse();
    let mut sum = 0;
    for (i, digit) in digits.iter().enumerate() {
        if !digit.is_digit(10) {
            return false;
        }
        let digit = digit.to_digit(10).unwrap();
        if i % 2 == 0 {
            sum += digit;
        } else {
            let doubled = digit * 2;
            if doubled > 9 {
                sum += doubled - 9;
            } else {
                sum += doubled;
            }
        }
    }
    if digits.len() < 2 {
        return false;
    }
    sum % 10 == 0
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}
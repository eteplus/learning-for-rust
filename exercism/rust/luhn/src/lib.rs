/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
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

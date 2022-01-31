#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }
    let mut sum = 1;
    let mut i = 2;
    let mut j = num;
    while i < j {
        if num % i == 0 {
            sum += i;
            if num / i != i {
                sum += num/ i;
            }
            j = num / i;
        }
        i += 1;
    }
    match sum {
        n if n > num => Some(Classification::Abundant),
        n if n < num => Some(Classification::Deficient),
        _ => Some(Classification::Perfect),
    }
}

use std::collections::HashMap;
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = input.split("==").collect::<Vec<&str>>();
    if input.len() != 2 {
        return None;
    }
    let summands = input[0].trim().split("+").map(|x| x.trim())
                           .collect::<Vec<&str>>();
    let sum = input[1].trim();
    let mut letters = (summands.join("") + sum).chars().collect::<Vec<char>>();
    letters.sort();
    letters.dedup();
    if letters.len() > 10 {
        return None;
    }
    let mut coeffs = vec![0i64; letters.len()];
    for s in summands.iter() {
        accumulate(&mut coeffs, &letters, s, 1);
    }
    accumulate(&mut coeffs, &letters, sum, -1);
    let n = (0..letters.len()).map(|i| 10-i).fold(1, |a, b| a * b);
    for i in 0..n {
        let mut nums = (0..10).collect::<Vec<u8>>();
        let mut m = i;
        let mut zero = None;
        for k in 0..letters.len() {
            let r = m % (10 - k);
            m = m / (10 - k);
            let tmp = nums[k];
            nums[k] = nums[r + k];
            nums[r + k] = tmp;
            if nums[k] == 0 {
                zero = Some(letters[k]);
            }
        }
        if let Some(c) = zero {
            if summands.iter().any(|s| s.starts_with(c)) || sum.starts_with(c) {
                continue;
            }
        }
        if coeffs.iter().zip(nums.iter().take(letters.len()))
                 .map(|(c, n)| c * (*n as i64))
                 .sum::<i64>() == 0 {
            return Some(letters.iter().map(|c| *c)
                               .zip(nums.iter().map(|n| *n)
                                   .take(letters.len()))
                               .collect::<HashMap<_, _>>());
        }
    }
    None
}
fn accumulate(coeffs: &mut Vec<i64>, letters: &Vec<char>, s: &str, m: i64) {
    let mut power = 1i64;
    for c in s.chars().rev() {
        let i = letters.iter().position(|&a| a == c).unwrap();
        coeffs[i] += power * m;
        power *= 10;
    }
}
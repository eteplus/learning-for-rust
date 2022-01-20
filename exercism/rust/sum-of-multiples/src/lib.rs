pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for num in 1..limit {
        for factor in factors {
            if *factor != 0 && num % factor == 0 {
                sum += num;
                break;
            }
        }
    }
    sum
}

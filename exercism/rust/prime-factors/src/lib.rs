pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
     while n > 1 {
        let i = (2..n+1).find(|x| n % x == 0 ).unwrap();
        prime_factors.push(i);
        n /= i;
    }
    prime_factors
}

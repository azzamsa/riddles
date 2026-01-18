// Find the prime factors

pub fn is_prime(n: u32) -> bool {
    // To check if a number is prime,
    // you only need to check for divisors up to its square root.
    // Any divisor larger than the square root would have a corresponding smaller divisor.
    let sqrt_n = (n as f32).sqrt() as u32;
    (2..=sqrt_n).all(|x| n % x != 0)
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors_: Vec<u64> = Vec::new();

    for x in 2..n.find(|x| n % x == 0) {
        if is_prime(x) {
            let tmp = n / x;
            if tmp == 0 {
                factors_.push(x);
                factors(tmp);
            } else {
                continue;
            }
        }
    }
    factors_
}

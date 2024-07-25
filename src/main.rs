use num_bigint::BigUint;
use num_traits::{One, Zero};
use num_integer::Integer;
use rayon::prelude::*;//parallel processing
use std::str::FromStr;
use std::time::Instant;

// Function to determine if a number is prime.
fn is_prime(n: &BigUint) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    let two = BigUint::from(2u32);
    if n == &two {
        return true;
    }
    if n.is_even() {
        return false;
    }

    let mut i = BigUint::from(3u32);
    let limit = sqrt(n) + &BigUint::one();
    while &i < &limit {
        if n % &i == BigUint::zero() {
            return false;
        }
        i += &two;
    }
    true
}

// Function for computing the integer square root using the Newton method.
fn sqrt(n: &BigUint) -> BigUint {
    let mut x0: BigUint = n.clone();
    let mut x1: BigUint = (n >> 1) + BigUint::one();
    while x1 < x0 {
        x0 = x1.clone();
        x1 = (n / &x1 + &x1) >> 1;
    }
    x0
}

fn main() {
    let lower_str = "15678";
    let upper_str ="345678";

    let lower = BigUint::from_str(lower_str).unwrap();
    let upper = BigUint::from_str(upper_str).unwrap();

    println!("Finding primes between {} and {}", lower_str, upper_str);
    let start = Instant::now();

    // Convert the BigUint range to a Vec<BigUint> for parallel processing
    let mut numbers = Vec::new();
    let mut num = lower.clone();
    while num <= upper {
        numbers.push(num.clone());
        num += BigUint::one();
    }

    // Find primes in parallel
    let primes: Vec<BigUint> = numbers
        .into_par_iter()
        .filter(|num| is_prime(num))
        .collect();

    for prime in primes {
        println!("{}", prime);
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}



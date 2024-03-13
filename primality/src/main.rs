use utils::{get_i64, Prng};

const NUM_TESTS: i64 = 20;

fn is_probably_prime(num_tests: i64) -> f64 {
    (1.0 - 0.5f64.powi(num_tests as i32)) * 100.0
}

fn find_prime(prng: &mut Prng, min: i64, max: i64, num_tests: i64) -> i64 {
    loop {
        let num = prng.next_i64(min, max);
        if is_prime(prng, num, num_tests) {
            return num;
        }
    }
}

fn is_prime(prng: &mut Prng, p: i64, num_tests: i64) -> bool {
    for _ in 0..num_tests {
        let n = prng.next_i64(2, p - 1);
        if fast_exp_mod(n, p - 1, p) != 1 {
            return false;
        }
    }
    true
}

fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result = 1;
    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * num) % modulus;
        }
        num = (num * num) % modulus;
        pow /= 2;
    }
    result
}

fn main() {
    // Prepare a Prng.
    let mut prng = Prng::new();

    // Display the probability that a number is prime
    // if it passes all NUM_TESTS tests.
    let probability = is_probably_prime(NUM_TESTS);
    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 {
            break;
        }

        // Calculate minimum and maximum values.
        let mut min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 {
            min = 2;
        } // 1 is not prime.

        // Find a prime.
        println!("Prime: {}", find_prime(&mut prng, min, max, NUM_TESTS));
    }
}

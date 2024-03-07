use utils::get_i64;

// Perform fast exponentiation.
fn fast_exp(mut num: i64, mut pow: i64) -> i64 {
    let mut result = 1;
    while pow > 0 {
        if pow % 2 == 1 {
            result *= num;
        }
        num *= num;
        pow /= 2;
    }
    result
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
    loop {
        let num = get_i64("Enter the base (<= 0 to exit): ");
        if num <= 0 {
            break;
        }

        let pow = get_i64("Enter the exponent: ");
        let modulus = get_i64("Enter the modulus: ");

        let result = fast_exp(num, pow);
        println!("Fast Exponentiation: {}", result);
        println!("{num}^{pow} = {}", num.pow(pow as u32));

        let result_mod = fast_exp_mod(num, pow, modulus);
        println!("FastExp. (mod {}): {}", modulus, result_mod);
        println!(
            "{num}^{pow} % {modulus} = {}",
            num.pow(pow as u32) % modulus
        );
    }
}

use std::io::{self, Write};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>().expect("Error parsing integer");
}

fn main() {
    loop {
        let a = get_i64("Enter a number: ");
        if a < 1 {
            break;
        }
        let b = get_i64("Enter another number: ");
        if b < 1 {
            break;
        }
        println!(
            "The greatest common divisor of {a} and {b} is {}",
            gcd(a, b)
        );
        println!("The least common multiple of {a} and {b} is {}", lcm(a, b));
        println!()
    }
}

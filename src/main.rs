use std::io::{stdin, stdout, Result, Write};

fn prompt_number(prompt: &str) -> Option<i32> {
    let mut option: String = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    if let Ok(_) = stdin().read_line(&mut option) {
        option.truncate(option.trim_end().len());
        if let Ok(number) = option.parse() {
            Option::Some(number)
        } else {
            println!("Failed to parse number {}", option);
            Option::None
        }
    } else {
        println!("Failed to get input from user");
        Option::None
    }
}

fn task_fibonacci() {
    let end = 10_000_000;
    let mut fib_nums = (0, 1);
    while fib_nums.0 <= end {
        println!("{}", fib_nums.0);
        fib_nums = (fib_nums.1, fib_nums.0 + fib_nums.1);
    };
}

fn is_prime(num: i32) -> bool {
    let limit = f32::sqrt(num as f32) as i32;
    for div in 2..limit {
        if num % div == 0 {
            return false;
        }
    }
    return true;
}

fn task_primes() {
    let start = 2;
    let end = 1_000_000;
    for i in start..=end {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn get_digits(num: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::<>::new();
    let mut num_aux = num;
    while num_aux > 0 {
        let digit = num_aux % 10;
        result.push(digit);
        num_aux /= 10;
    }
    result.reverse();
    return result;
}
/*
A narcissistic number or an Armstrong number is a natural
number that is equal to the sum of its own digits each raised to
the power of the number of its digits.
 */
fn is_armstrong(num: i32) -> bool {
    let digits = get_digits(num);
    let mut sum = 0;
    for digit in &digits {
        sum += digit.pow(digits.len() as u32); 
    }
    return sum == num;
}
/*
Output all Armstrong numbers in the range from 10 to
1,000,000 to the screen.
 */
fn task_narcissistic() {
    for i in 10..=1_000_000 {
        if is_armstrong(i) {
            println!("{}", i);
        }
    }
}

fn get_divisor_sum(num: i32) -> i32 {
    let mut divisor_sum = 0;
    let square_root = f32::sqrt(num as f32).floor() as i32;
    for i in 1..=square_root {
        if num % i == 0 {
            divisor_sum += i;
            if i > 1 {
                divisor_sum += num / i;
            }
        }
    }
    return divisor_sum;
}
/*
A perfect number is a natural number that is equal to the sum
of all its own divisors (that is, all positive divisors different from
the number itself).
Display all the perfect numbers in the range from 0 to 1,000,000.
*/
fn task_perfect() {
    for i in 0..=1_000_000 {
        if i == get_divisor_sum(i) {
            println!("{}", i);
        }
    }
}

fn task_palindrome_number() {
    if let Some(number) = prompt_number("Input a number: ") {
        let digits = get_digits(number);
        let mut new_number = 0;
        let mut base = 1;
        for d in digits {
            new_number += d * base;
            base *= 10;
        }
        if new_number == number {
            println!("{} is a palindrome number.", number);
        } else {
            println!("{} is **not** a palindrome number", number);
        }
    } else {
        println!("Invalid number");
    }
}

fn main() -> Result<()> {
    println!("Input exercise to run: ");
    let mut option: String = String::new();
    stdin().read_line(&mut option)?;
    option.truncate(option.trim_end().len());
    match option.as_str() {
        "fibonacci" => task_fibonacci(),
        "primes" => task_primes(),
        "narcissistic" => task_narcissistic(),
        "perfect" => task_perfect(),
        "palindrome_number" => task_palindrome_number(),
        _ => println!("Invalid option"),
    }
    Ok(())
}

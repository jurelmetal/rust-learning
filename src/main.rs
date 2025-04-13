use std::io::{stdin, Result};
mod tasks_loops;
mod common;
mod tasks_nested_loops;

use tasks_loops::*;
use tasks_nested_loops::*;

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
        "eight_digits" => task_eight_digit_no_repeat(),
        "bit_rep" => task_bit_rep(),
        "symmetric_clock" => task_clock_combinations(),
        "sum_of_multiples_3_5" => task_multiples_of_3_or_5(),
        "smallest_divisible_upto_20" => task_smallest_divisible_upto_20(),
        "fizzbuzz" => task_fizzbuzz(),
        "athlete" => task_athlete(),
        "triangle1" => task_triangle_down_right(),
        "triangle2" => task_triangle_up_right(),
        _ => println!("Invalid option"),
    }
    Ok(())
}

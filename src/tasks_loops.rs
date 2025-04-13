use std::io::{stdout, Write};
use crate::common::prompt_number;

pub fn task_fibonacci() {
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

pub fn task_primes() {
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
pub fn task_narcissistic() {
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
pub fn task_perfect() {
    for i in 0..=1_000_000 {
        if i == get_divisor_sum(i) {
            println!("{}", i);
        }
    }
}

pub fn task_palindrome_number() {
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

fn has_repeated_digits(num: i32) -> bool {
    let digits = get_digits(num);
    for i in 0..digits.len() {
        for j in i..digits.len() {
            if i != j && digits[i] == digits[j] {
                return true;
            }
        }
    }
    return false;
}
/*
Task 6
Output all eight-digit numbers, in which digits do not repeat, to
the console. These numbers should be divisible by 12345 without
a remainder. Show the total number of numbers found.
 */
pub fn task_eight_digit_no_repeat() {
    let mut current: i32 = 12345 * (10000000.0_f32 / 12345.0_f32).floor() as i32;
    while current <= 99999999 {
        if current >= 10000000 {
            // check if no repeated digits
            if !has_repeated_digits(current) {
                println!("{}", current);
            }
        }
        current += 12345;
    }
}

/*
Show bit representation of the value of a variable of the int type,
using only one loop, a control variable, output to the console, and
bit operations.

Do not use strings and any other ready-made functions (methods).
 */
pub fn task_bit_rep() {
    if let Some(number) = prompt_number("Input an integer: ") {
        // bit representation of number
        let mut bit_position = 31;
        let mut first_one_found = false;
        while bit_position >= 0 {
            if (1 << bit_position) & number > 0 {
                print!("1");
                first_one_found = true;
            } else if first_one_found {
                print!("0");
            }
            bit_position -= 1;
        }
        let _ = stdout().flush();
    } else {
        println!("Invalid number");
    }
}

/*
The electronic clock shows time in the format from 00:00 to
23:59. Write a program that will output to the console, how many
times a day it happens that a symmetrical combination on the left
from the colon is shown for the combination on the right from the
colon (for example, 02:20, 11:11 or 15:51). Display all symmetrical
combinations. Print the total number of combinations.
 */
pub fn task_clock_combinations() {
    let mut total_combinations = 0;
    for i in 0..23 {
        let reverse = (i / 10) + (i % 10) * 10;
        if reverse < 60 {
            println!("Valid symmetric clock combination: {:02}:{:02}", i, reverse);
            total_combinations += 1;
        }
    }
    println!("Total combinations: {}", total_combinations);
}

/*
If we list all natural numbers less than 10, multiples of 3 or 5,
we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all numbers that are multiples of 3 or 5 starting
from 0 and up to 1000.
 */
pub fn task_multiples_of_3_or_5() {
    let mut total = 0;
    for i in 0..=1000 {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }
    println!("Sum of all multiples of 3 or 5: {}", total);
}

/*
2520 is the smallest number that can be divided into each of
the numbers from 1 to 10 without a remainder. Write a program
that calculates the smallest positive number, which is divided by
all numbers from 1 to 20.
 */

 pub fn task_smallest_divisible_upto_20() {
    let divisors = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    for candidate in (11*20)..i32::MAX {
        let mut succeed = true;
        for d in divisors {
            if candidate % d != 0 {
                succeed = false;
                break;
            }
        }
        if succeed {
            println!("Found smallest candidate: {}", candidate);
            break;
        }
    }
 }

 /*
 Write a program that displays numbers from 1 to 1000. In this
case, instead of numbers that are multiples of three, the program
should  output  the  word  fizz,  and  instead  of  numbers  that  are
multiples  of  five,  the  word  buzz  should  be  outputted.  If  the
number is a multiple of fifteen, then the program should output
the word hiss instead of the number.
  */
pub fn task_fizzbuzz() {
    for i in 1..=1000 {
        if i % 15 == 0 {
            println!("hiss");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

/*
The first day an athlete ran x kilometers, and then every day he
increased the mileage by 10% from the previous value. According
to  the  number  y  indicated  from  the  keyboard,  determine  the
number of the day, when the athlete’s mileage will be at least y
kilometers.
 */
pub fn task_athlete() {
    let x = prompt_number("Enter the number of km in the first day: ").unwrap();
    let y = prompt_number("Enter the number of km in the final run: ").unwrap();
    // Formula is
    // km(day) = x * (1.1) ^ day
    // so if we know km(day) = y we can find day
    // y = x * 1.1 ^ day
    // y / x = 1.1 ^ day
    // log (y / x) = day * log(1.1)
    // log (y / x) / log(1.1) = day
    // log_base1.1(y/x)
    let day = f32::log(y as f32 / x as f32, 1.1).ceil();
    let real_km = x as f32 * 1.1_f32.powf(day);
    println!("The day the athlete runs {} km or more is day {}, running {} km", y, day, real_km);
}
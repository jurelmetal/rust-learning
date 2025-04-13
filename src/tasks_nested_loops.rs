use crate::common::prompt_number;

pub fn task_triangle_down_right() {
    let height = prompt_number("Triangle height: ").unwrap();
    for row in 0..height {
        for col in 0..height {
            if col == height - 1 - row ||
            col == height - 1 ||
            row == height - 1
            {
                print!("*")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn task_triangle_up_right() {
    let height = prompt_number("Triangle height: ").unwrap();
    for row in 0..height {
        for col in 0..height {
            if col == row ||
            col == height - 1 ||
            row == 0
            {
                print!("*")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
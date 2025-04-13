use std::io::{stdin, stdout, Write};

pub fn prompt_number(prompt: &str) -> Option<i32> {
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
use colored::*;
use std::io::{self, Write};

pub fn prompt() -> String {
    let mut input = String::new();

    println!();
    print!("{}", "❯ ".truecolor(81, 159, 80));
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("❌ unable to read user input");

    return input.trim().to_string();
}

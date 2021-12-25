use std::io::Write;
use std::{io, process};

use termion::input::TermRead;

pub fn prompt_year(stdin: &mut io::StdinLock) -> u16 {
    println!("Please enter the year for which you would like to create tasks.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        if let Ok(Some(year)) = stdin.read_line() {
            if let Ok(year) = year.parse::<u16>() {
                println!();
                return year;
            } else {
                println!("Invalid value provided: '{}'. Please try again.", year);
            }
        } else {
            eprintln!("Unable to process your input. Aborting the program.");
            process::exit(1);
        }
    }
}

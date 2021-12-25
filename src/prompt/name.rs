use std::io::Write;
use std::{io, process};

use termion::input::TermRead;

pub fn prompt_name(stdin: &mut io::StdinLock) -> String {
    loop {
        println!("Please pick a name:");
        print!("> ");
        io::stdout().flush().unwrap();

        if let Ok(Some(name)) = stdin.read_line() {
            if name.is_empty() {
                print!("The name may not be empty. ");
            } else {
                return name;
            }
        } else {
            eprintln!("Unable to process your input. Aborting the program.");
            process::exit(1);
        }
    }
}

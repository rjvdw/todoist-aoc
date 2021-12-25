use std::io::Write;
use std::{env, io, process};

use termion::input::TermRead;

pub fn prompt_token(stdin: &mut io::StdinLock) -> String {
    if let Ok(token) = env::var("TODOIST_API_TOKEN") {
        println!("Using Todoist API token from environment.");
        println!();
        return token;
    }

    print!("In order for this script to create tasks, it will need to have access to your Todoist account. ");
    print!("For this, a token is needed. ");
    print!("This token can be found in the integrations settings view of the Todoist Web App ");
    print!("(https://todoist.com/prefs/integrations). ");
    print!("Alternatively, you can set this token in the environment variable: ");
    print!("TODOIST_API_TOKEN.");
    println!();
    println!();
    println!("This prompt will not show what you are typing.");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(b"> ").unwrap();
    stdout.flush().unwrap();

    if let Ok(Some(token)) = stdin.read_passwd(&mut stdout) {
        stdout.write_all(b"\n\n").unwrap();
        stdout.flush().unwrap();
        token
    } else {
        eprintln!("Unable to process your input. Aborting the program.");
        process::exit(1);
    }
}

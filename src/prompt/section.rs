use std::io::Write;
use std::{io, process};

use termion::input::TermRead;

use crate::todoist::section::{GetSectionsResponse, Section};

#[derive(Debug)]
pub enum SectionChoice {
    None,
    Some(Section),
    CreateNew,
}

pub fn choose_section(stdin: &mut io::StdinLock, sections: GetSectionsResponse) -> SectionChoice {
    print!("Please choose the section in which to create the tasks. ");

    loop {
        println!("The chosen project currently has the following sections:");
        for (idx, section) in sections.iter().enumerate() {
            println!("{}: {}", idx + 1, section.name);
        }
        println!("{}: Create new section.", sections.len() + 1);
        println!("{}: Do not use a section.", sections.len() + 2);
        println!("Please choose an option.");
        print!("> ");
        io::stdout().flush().unwrap();

        if let Ok(Some(idx)) = stdin.read_line() {
            if let Ok(idx) = idx.parse::<usize>() {
                if idx <= sections.len() {
                    println!(
                        "Okay, creating the tasks in section '{}'.",
                        sections[idx - 1].name,
                    );
                    println!();
                    return SectionChoice::Some(sections[idx - 1].clone());
                }

                #[allow(clippy::single_match)]
                match idx - sections.len() {
                    1 => {
                        println!(
                            "Okay, creating a new section and creating the tasks in that section."
                        );
                        return SectionChoice::CreateNew;
                    }
                    2 => {
                        println!("Okay, not creating the tasks inside a section.");
                        return SectionChoice::None;
                    }
                    _ => {}
                }
            }

            println!("Invalid value provided: '{}'. Please try again", idx);
        } else {
            eprintln!("Unable to process your input. Aborting the program.");
            process::exit(1);
        }
    }
}

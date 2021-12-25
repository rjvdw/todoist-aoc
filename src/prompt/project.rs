use std::io::Write;
use std::{io, process};

use termion::input::TermRead;

use crate::todoist::project::{GetProjectsResponse, Project};

#[derive(Debug)]
pub enum ProjectChoice {
    Some(Project),
    CreateNew,
}

pub fn choose_project(stdin: &mut io::StdinLock, projects: GetProjectsResponse) -> ProjectChoice {
    print!("Please choose a project in which to create the tasks. ");

    loop {
        println!("You currently have the following projects:");
        for (idx, project) in projects.iter().enumerate() {
            println!("{}: {}", idx + 1, project.name);
        }
        println!("{}: Create new project.", projects.len() + 1);
        println!("Please choose an option.");
        print!("> ");
        io::stdout().flush().unwrap();

        if let Ok(Some(idx)) = stdin.read_line() {
            if let Ok(idx) = idx.parse::<usize>() {
                if idx <= projects.len() {
                    println!(
                        "Okay, creating the tasks in project '{}'.",
                        projects[idx - 1].name,
                    );
                    println!();
                    return ProjectChoice::Some(projects[idx - 1].clone());
                }

                #[allow(clippy::single_match)]
                match idx - projects.len() {
                    1 => {
                        println!(
                            "Okay, creating a new project and creating the tasks in that project."
                        );
                        return ProjectChoice::CreateNew;
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

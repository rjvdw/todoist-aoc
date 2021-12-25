use std::io;

use project::choose_project;
use section::choose_section;

use crate::project::ProjectChoice;
use crate::prompt::token::prompt_token;
use crate::prompt::year::prompt_year;
use crate::prompt::{project, section};
use crate::section::SectionChoice;
use crate::util::BoxedResult;

mod prompt;
mod todoist;
pub mod util;

#[tokio::main]
async fn main() -> BoxedResult<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    println!("Welcome. This script will help you create tasks for Advent of Code in Todoist.");
    println!();

    let token = prompt_token(&mut stdin);
    let year = prompt_year(&mut stdin);

    let api = todoist::RestApi::new(token);

    let projects = api.fetch_projects().await?;
    let project = choose_project(&mut stdin, projects);

    let project = match project {
        ProjectChoice::Some(project) => project,
        ProjectChoice::CreateNew(name) => api.create_project(name).await?,
    };

    let sections = api.fetch_sections(project.id).await?;
    let section = choose_section(&mut stdin, sections);

    let section = match section {
        SectionChoice::None => None,
        SectionChoice::Some(section) => Some(section),
        SectionChoice::CreateNew(name) => Some(api.create_section(project.id, name).await?),
    };

    println!();
    for day in 1..=25 {
        println!("Creating a task for day {}...", day);
        api.create_task(&project, &section, year, day).await?;
    }
    println!();
    println!("All done! Have fun in {}! 🙂", year);

    Ok(())
}

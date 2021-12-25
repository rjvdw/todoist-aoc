use serde::{Deserialize, Serialize};

use crate::todoist::project::Project;
use crate::todoist::section::Section;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    content: String,
    project_id: u64,
    section_id: Option<u64>,
    due_datetime: String,
}

impl Task {
    pub fn new(project: &Project, section: &Option<Section>, year: u16, day: u8) -> Task {
        Task {
            content: format!(
                "[Advent of Code {year} - Day {day}](https://adventofcode.com/{year}/day/{day})",
                year = year,
                day = day,
            ),
            project_id: project.id,
            section_id: section.as_ref().map(|section| section.id),
            due_datetime: format!("{year}-12-{day:02}T05:00:00Z", year = year, day = day),
        }
    }
}

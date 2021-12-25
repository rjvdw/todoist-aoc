use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub url: String,
}

pub type GetProjectsResponse = Vec<Project>;

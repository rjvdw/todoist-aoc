//! Methods for accessing the Todoist API.

use reqwest::RequestBuilder;
use serde::Serialize;

use crate::todoist::project::{GetProjectsResponse, Project};
use crate::todoist::section::{GetSectionsResponse, Section};
use crate::util::BoxedResult;

pub mod project;
pub mod section;

const BASE_API_URL: &str = "https://api.todoist.com/rest/v1";

pub struct RestApi {
    token: String,
    client: reqwest::Client,
}

impl RestApi {
    pub fn new(token: String) -> RestApi {
        RestApi {
            token,
            client: reqwest::Client::new(),
        }
    }

    fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(&format!("{}/{}", BASE_API_URL, endpoint))
            .bearer_auth(&self.token)
    }

    fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .post(&format!("{}/{}", BASE_API_URL, endpoint))
            .bearer_auth(&self.token)
    }

    pub async fn fetch_projects(&self) -> BoxedResult<GetProjectsResponse> {
        let response = self.get("projects").send().await?;

        if !response.status().is_success() {
            eprintln!("{:?}", response);
        }

        Ok(response.json::<GetProjectsResponse>().await?)
    }

    pub async fn create_project(&self, name: String) -> BoxedResult<Project> {
        #[derive(Serialize)]
        struct Request {
            name: String,
        }

        let request = Request { name };

        let response = self.post("projects").json(&request).send().await?;

        if !response.status().is_success() {
            eprintln!("{:?}", response);
        }

        Ok(response.json::<Project>().await?)
    }

    pub async fn fetch_sections(&self, project_id: u64) -> BoxedResult<GetSectionsResponse> {
        let response = self
            .get(&format!("sections?project_id={}", project_id))
            .send()
            .await?;

        if !response.status().is_success() {
            eprintln!("{:?}", response);
        }

        Ok(response.json::<GetSectionsResponse>().await?)
    }

    pub async fn create_section(&self, project_id: u64, name: String) -> BoxedResult<Section> {
        #[derive(Serialize)]
        struct Request {
            name: String,
            project_id: u64,
        }

        let request = Request { name, project_id };

        let response = self.post("sections").json(&request).send().await?;

        if !response.status().is_success() {
            eprintln!("{:?}", response);
        }

        Ok(response.json::<Section>().await?)
    }
}

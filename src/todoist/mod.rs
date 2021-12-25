//! Methods for accessing the Todoist API.

use reqwest::{RequestBuilder, Response};
use serde::Serialize;

use crate::todoist::project::{GetProjectsResponse, Project};
use crate::todoist::section::{GetSectionsResponse, Section};
use crate::todoist::task::Task;
use crate::util::{BoxedResult, HttpError};

pub mod project;
pub mod section;
mod task;

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

    async fn check_if_error_response(&self, response: Response) -> BoxedResult<Response> {
        if !response.status().is_success() {
            Err(Box::new(HttpError {
                status: response.status().to_string(),
                message: response.text().await?,
            }))
        } else {
            Ok(response)
        }
    }

    pub async fn fetch_projects(&self) -> BoxedResult<GetProjectsResponse> {
        let response = self.get("projects").send().await?;
        let response = self.check_if_error_response(response).await?;

        Ok(response.json::<GetProjectsResponse>().await?)
    }

    pub async fn create_project(&self, name: String) -> BoxedResult<Project> {
        #[derive(Serialize)]
        struct Request {
            name: String,
        }

        let request = Request { name };

        let response = self.post("projects").json(&request).send().await?;
        let response = self.check_if_error_response(response).await?;

        Ok(response.json::<Project>().await?)
    }

    pub async fn fetch_sections(&self, project_id: u64) -> BoxedResult<GetSectionsResponse> {
        let response = self
            .get(&format!("sections?project_id={}", project_id))
            .send()
            .await?;
        let response = self.check_if_error_response(response).await?;

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
        let response = self.check_if_error_response(response).await?;

        Ok(response.json::<Section>().await?)
    }

    pub async fn create_task(
        &self,
        project: &Project,
        section: &Option<Section>,
        year: u16,
        day: u8,
    ) -> BoxedResult<()> {
        let request = Task::new(project, section, year, day);

        let response = self.post("tasks").json(&request).send().await?;
        let _ = self.check_if_error_response(response).await?;

        Ok(())
    }
}

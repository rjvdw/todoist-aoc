//! Methods for accessing the Todoist API.

use crate::todoist::project::GetProjectsResponse;
use crate::todoist::section::GetSectionsResponse;
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

    pub async fn fetch_projects(&self) -> BoxedResult<GetProjectsResponse> {
        Ok(self
            .client
            .get(&format!("{}/projects", BASE_API_URL))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<GetProjectsResponse>()
            .await?)
    }

    pub async fn fetch_sections(&self, project_id: u64) -> BoxedResult<GetSectionsResponse> {
        Ok(self
            .client
            .get(&format!(
                "{}/sections?project_id={}",
                BASE_API_URL, project_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<GetSectionsResponse>()
            .await?)
    }
}

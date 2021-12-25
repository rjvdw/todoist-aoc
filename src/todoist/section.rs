use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Section {
    pub id: u64,
    pub name: String,
}

pub type GetSectionsResponse = Vec<Section>;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct Schema {
    pub provider: Provider,

    pub types: Vec<CommitType>,

    pub common_scopes: Option<Vec<String>>,

    #[schemars(description = "available variables: ${type} ${scope} ${mr} ${issues}")]
    #[serde(rename = "titleFormat")]
    pub title_format: Option<String>,

    #[serde(rename = "branchNameFormat")]
    pub branch_name_format: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub enum Provider {
    #[serde(rename = "github-actions")]
    GitHubActions,

    #[serde(rename = "gitlab-runner")]
    GitLabRunner,

    #[serde(rename = "type")]
    Custom,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct CommitType {
    #[serde(rename = "type")]
    pub type_name: String,

    pub scopes: Option<Vec<String>>,
}

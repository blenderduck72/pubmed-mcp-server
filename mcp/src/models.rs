use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SearchArgs {
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "u32")]
    pub retstart: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SearchResponse {
    pub content: String,
    pub ret_start: u32,
    pub ret_max: u32,
}

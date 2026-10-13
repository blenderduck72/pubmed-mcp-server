use client::errors::ClientError;
use rmcp::ErrorData as McpError;
use rmcp::model::ErrorCode;

pub struct McpClientError(pub ClientError);

impl From<McpClientError> for McpError {
    fn from(value: McpClientError) -> McpError {
        match value.0 {
            ClientError::InitiationHeader => McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Internal Error With Header Construction",
                None,
            ),
            ClientError::InitiationReqwestClient(_e) => McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Internal Error With Request Client Construction",
                None,
            ),
            ClientError::FetchReqwest(e) | ClientError::SearchReqwest(e) => {
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    "Failed To Retrieve PubMed Article",
                    Some(
                        serde_json::json!(
                            {
                                "status": &e.status().map(|v| v.to_string()),
                                "url": &e.url().map(|v| {v.to_string()})
                            }
                        ),
                    ),
                )
            }
            ClientError::SearchSerialization(e) => McpError::new(
                ErrorCode::PARSE_ERROR,
                "Failed to Deserialize XML Search Results from PubMed",
                Some(
                    serde_json::json!(
                        {
                            "url": e.url().map(|v| {v.to_string()})
                        }
                    ),
                ),
            ),
            ClientError::FetchSerialization(e) => McpError::new(
                ErrorCode::PARSE_ERROR,
                "Failed To Deserialize XML Fetch Results From PubMed",
                Some(
                    serde_json::json! {
                        {
                            "url": e.url().map(|v|{v.to_string()})
                        }
                    },
                ),
            ),
            ClientError::ParseFetchXml(e) => McpError::new(
                ErrorCode::PARSE_ERROR,
                "Failed To Deserialze XML to JSON for PubMed Fetch Results",
                Some(
                    serde_json::json!(
                        {
                            "info": format!("{}",e)
                        }
                    ),
                ),
            ),
            ClientError::ParseSearchXml(e) => McpError::new(
                ErrorCode::PARSE_ERROR,
                "Failed To Deserialze XML to JSON for PubMed Search Results",
                Some(
                    serde_json::json!(
                        {
                            "info": format!("{}",e)
                        }
                    ),
                ),
            ),
        }
    }
}

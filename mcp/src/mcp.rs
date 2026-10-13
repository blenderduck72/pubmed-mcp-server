use super::errors::McpClientError;
use super::models;
use client::{Client, utils};
use rmcp::model::CallToolResult;
use rmcp::model::ErrorCode;
use rmcp::{
    ErrorData as McpError,
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    // model::ContentBlock,
};

#[derive(Clone)]
pub struct Mcp {
    client: Client,
    tool_router: ToolRouter<Self>,
}

#[rmcp::tool_router]
impl Mcp {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }

    #[rmcp::tool(
        description = "Searches PubMed using an Essie Syntax Expression, and returns a list of abstracts",
        name = "PubMed Search"
    )]
    pub async fn search(
        &self,
        Parameters(params): Parameters<models::SearchArgs>,
    ) -> Result<CallToolResult, McpError> {
        let results = self
            .client
            .search(
                &params.expression,
                params.retstart,
                Some(5),
            )
            .await
            .map_err(McpClientError)?;

        let ids: Vec<String> = results
            .id_list
            .ids
            .into_iter()
            .flat_map(|id| id.value)
            .collect();

        let article_set = self
            .client
            .fetch(&ids)
            .await
            .map_err(McpClientError)?;

        let response =
            utils::pub_med_article_set_to_markdown(article_set).await;

        let search_response = models::SearchResponse {
            ret_start: results.ret_start,
            content: response,
            ret_max: results.ret_max,
        };
        let value = serde_json::to_value(search_response).map_err(|_e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                "Internal Error Serializing Results",
                None,
            )
        })?;

        Ok(CallToolResult::structured(value))
    }
}

#[rmcp::tool_handler(router = self.tool_router)]
impl ServerHandler for Mcp {
    fn get_info(&self) -> rmcp::model::ServerConfig {
        let capabilities = rmcp::model::ServerCapabilities::builder()
            .enable_tools()
            .build();

        rmcp::model::ServerConfig::new(capabilities)
            .with_server_info(
                rmcp::model::Implementation::new(
                    "PubMed MCP Server",
                    "0.0.1",
                ),
            )
            .with_instructions(
                "Tools for retrieiving updated information from PubMed.",
            )
    }
}

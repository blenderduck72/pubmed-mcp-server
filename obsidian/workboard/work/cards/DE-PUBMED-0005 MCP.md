## Context
With our `Client` package ready with its types and commands it is time to setup our MCP server.

We want our MCP Server to combine our `client` packages search and retrieval commands to so that an LLM can make a query and return a list of relevant articles as markdown 

## Implementation Notes
This means that we will note be returning a fetch's full translated `json`, but rather taking certain fields and turning them into a markdown document list.

For this we are shooting for an Axum based MCP Server -- later in documentation we'll want to highlight the benefits of re-using the Tool Router (a tricky topic in the mcp documentation)

## Acceptance Criteria 
A functioning MCP Server with a search tool that returns articles text as markdown and can be ran via the mcp protocol inspector. 

## References
- [[Rust MCP Server Portfolio Project#DE-PUBMED-0005 MCP]]
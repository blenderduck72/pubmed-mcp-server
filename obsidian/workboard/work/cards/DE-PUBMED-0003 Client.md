## Context
With Rust Types defined it is time to organize some basic functionality -- specifically making requests to the Search & Fetch API. 

Ideally we want these request to leverage some basic defaults for pagination, but not much else. While the API support more options, this is meant to be more of an opinionated endeavor - the search will always use `retmode=json`, and targeted database will always be `db=pubmed`. 

The fetch call should return PubMed article data as structured JSON.

## Acceptance Criteria
The `client` create provides to functions for searching and fetching PubMed data. 

## References
- [[Rust MCP Server Portfolio Project#DE-PUBMED-0003 Client]]
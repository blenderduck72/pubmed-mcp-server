
# Goal
To showcase experience in developing Tools for API a proposed MCP Server for PubMed is being considered.

PubMed presents some interesting challenges for MCP Servers in that its API is strictly rate limited, its text an inconsistent XML. While there are other MCP Servers available for this many of them share similar faults: 
1. Contain no unit tests verifying/validating the codes underlying assumptions.
2. Return full API payloads of XML - this can cost a lot in terms of tokens. 
3. Is not entirely customizable

Building an MCP Server around PubMed without the use of AI in `Rust` can showcase familiarity with:
1. The `Rust` language as a whole including:
	- Syntax
	- Unit Tests
	- Familiarity with Popular Dependencies
2. Familiarity with the MCP Standard and some of its quirks

The example project does not have to be fully production ready, but it should aim to showcase the familiarity described above, and showcase ways that I work -- planning, experimenting, organizing, ticketing, and pushing. 

# Epic Plan (Quick Draft)

## DE-PUBMED-0001 | Initial
Basic ticket for setting up initial REPO & README. This will be a public repo so determining which GitHub account to use should be taken into consideration. 

## DE-PUBMED-0002 | Types
Setting up Initial Types - translating PubMed XML to Structured Types. We may leverage Rust's [quick-xml](https://crates.io/crates/quick-xml) crate to help map XML to Structs. Another issue that may arise is nested HTML can be in an Article's `Abstract` tag - something quick-xml may take issue with.

## DE-PUBMED-0003 | Client
Setting up Initial pattern for an MCP Client. This will involve setting up two commands 1 for Search and 1 for Retrieval. PubMed's Search API does provide JSON support; however, their fetch API only reliably provides XML support - so this should be translated into types.

While PubMed's API allows for an API key and we could allow that as an environment variable, we'll save that for a later option.

## DE-PUBMED-0004 | CLI
Setup a basic CLI to leverage the Client package as a means for quick testing and iteration. While showcasing testing is an ideal goal, PubMed and its structures have been around for years - it is feasible even mapping out types may not catch all edge cases, thus, having a quick way to validate our client against actual data from their API is a useful tool to break assumptions. 

## DE-PUBMED-0005 | MCP
Setup an MCP Server that uses the official Rust MCP crate [rmcp](https://crates.io/crates/rmcp), [axum](https://crates.io/crates/axum) to showcase how the MCP tool could be hosted via a server, and [clap](https://crates.io/crates/clap) with [console](https://crates.io/crates/console) for showcasing more cli skills. 

The goal of the MCP Server is to combine search and retrieval(fetch) so that an LLM can pass in a query, and retrieve basic article contents - title, abstract, and author.  

## DE-PUBMED-006 | Documentation
While I typically prefer to document as I go, the goal is to get something up quick. So reserving a Documentation & Cleanup task, but to also showcase a demo of sorts.
# Future Improvements
While not necessarily in the scope of getting something up, to make this more useful as a product and showcasing IaC & CI/CD skills some things to consider tagging on later:

## PubMed API Key
As stated this would be skipped initially, but adding it would be a smaller lift. 

## Image Mode
Most frontier models that would use a PubMed MCP Server support Images. The MCP tool protocol now allows for images responses. Given that PubMed articles are lengthy, it may be more cost effective token-wise to compress multiple articles in a an image or series of images and have the tool return this instead of markdown text. 

## CDK AWS Lambda Example
It may be worth showcasing an example of the MCP Server running on an AWS lambda utilizing the [lambda_runtime](https://crates.io/crates/lambda_runtime) or more appropriately [lambda_http](https://crates.io/crates/lambda_http).

## CI/CD
Setting up a simple GitHub Actions Pipeline for Review and running unit tests would show my familiarity with CI/CD practices - but GitHub Actions has been notoriously difficult to work with even more lately.
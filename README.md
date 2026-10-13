# PUB MED MCP Server

This repo contains an example PubMed MCP Server written in Rust.

Its meant to showcase the usage of a few concepts in Rust and how the Rust
MCP crate [rmcp](https://github.com/modelcontextprotocol/rust-sdk) can be
leveraged to quickly put together an MCP wrapper around an API.

I did quickly throw this project together by hand (no AI agents), but did use
AI to review offer suggestion on changes. Again this is just quick
implementation.

If you are looking to learn about PubMed's API or Rust MCP Servers,
this may be useful to you.

## Project Structure

This project is structured into 3 modules/packages:

- **[client](./client/README.md)** - which contains types and an API Service which faciliates REST
  requests to the PubMed API.
- **[mcp](./mcp/README.md)** - an Axum MCP Server which uses the `client` module, allowing an
  agent to make requests to PubMed.
- **[cli](./cli//README.md)** - a quick command line utility that leverages `client` to query
  PubMed and output its contents to the terminal.

## Working With PubMed's API

PubMed's API is not a typical REST API with resources, and thus has some
unique limitations:

1. **Rate Limit** - It has a Rate Limit of 5 requests per second, unless an
   API Key is utilized, at which case the rate of requests can be increased to
   10 Requests Per Second
   ([reference](https://www.ncbi.nlm.nih.gov/datasets/docs/v2/api/api-keys/))
2. **XML Responses** - though some of the endpoints from PubMed's API such as
   Search can return JSON, the endpoints that retrieve/fetch Article abstracts
   primarily favor XML.
3. **Essie Syntax** - PubMed's search endpoint utilizes the
   [Essie Syntax](https://pmc.ncbi.nlm.nih.gov/articles/PMC2244877/) to
   build advance queries. While PubMed does a decent
   [job of explaining](https://pubmed.ncbi.nlm.nih.gov/help/#how-do-i-search-pubmed) how to
   write queries in the Essie Syntax, I find ClinicalTrials.gov
   [primer](https://clinicaltrials.gov/find-studies/constructing-complex-search-queries)
   to be a bit more informative.
4. **Content** - A PubMedArticleSet can get large, because a PubMed Article can
   be large. Not every detail about a PubMed Article is needed, by an agent.
   Primarily most use cases desire the Abstract -- the text content describing
   what the article is about.

These limitiations are handled differently in other PubMed MCP Servers (and
there are a few of them out there). For example, `Content` presented from an
MCP Tool can contain more content from an Article but contain only part of an
Article's Abstract. This being a quick simple example can be improved upon and
customized to suit someones use case. A few ideas are listed below in the
[Future Improvements](#future-improvements) section.

## Personal Highlights

### Unit Tests

Most MCP Servers I've come across don't supply unit tests.In my experience
Production Software is not ready for relase if there is no unit tests.

## Future Improvements

While this MCP is great for a demo, there is a few areas it can be improved
upon or be productionalized.

## Development Process

I have included an [Obsidian vault](./obsidian/README.md) that
utilizes the Kanban plugin - to showcase
simple planning and ticketing.

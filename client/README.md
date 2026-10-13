# Client

This module is a shared library of resources for interacting with PubMed's API.

It can be used by other crates/modules within this project.

Internally, this modules is broken down into the following structure:

- **contants** - Hardcoded urls for PubMed API (organized this way to be lazy
  for Dependency Injection for unit tests).
- **client** - Private module that holds our service for interacting with the
  PubMed API
- **errors** - Structure describing the various errors for for communicating
  with PubMed's API.
- **models** - Rust Types that describe PubMed Search responses & turns them
  into them JSON serializable data structures.
- **utils** - Tools that turn PubMed Rust Types into Markdown text.

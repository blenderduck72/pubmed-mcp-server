# Client

This module is a shared library of resources for interacting with PubMed's API.

It can be used by other crates/modules within this project.

Internally, this modules is broken down into the following structure:

- **models** - Rust Types that describe PubMed Search responses & turns them
  into
- **utils** - Tools that turn PubMed Rust Types into Markdown text.

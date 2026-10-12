## Context
A key aspect of our MCP server will be to translate PubMed's XML into Rust data structures. 

While PubMed's main search[ API via Entrez](https://www.ncbi.nlm.nih.gov/home/develop/api/) can accept JSON via its [retmode](https://www.ncbi.nlm.nih.gov/books/NBK25500/#chapter1.Searching_a_Database) parameter, the fetch API is lacking. While, technically, fetch [supports](https://www.ncbi.nlm.nih.gov/books/NBK25500/#chapter1.Downloading_Full_Records) a `retmode=json` parameter, it does not return full article content -- just ids. 

Confusingly the Fetch API's XML [DTD](https://www.nlm.nih.gov/bsd/licensee/data_elements_doc.html) (Document Type Definition) links to previous pages in the [wayback](https://wayback.archive-it.org/org-350/20240424200207/https://www.nlm.nih.gov/bsd/licensee/elements_alphabetical.html) machine as opposed providing the most updated DTD. 

Primarily our MCP tool will want to show basic information about an article, title, abstract, author and status; however, other fields can be mapped so if cloned/ forked etc, others can customize it to their needs.

## Implementation Notes
It can be possible to use Rust's [quick-xml](https://crates.io/crates/quick-xml) crate along with [serde](https://crates.io/crates/serde) to define types. Tricky parts may involve the `Abstract` tag and other xml tags that allow for nested HTML.  

## Acceptance Criteria

A basic series of Rust Types that can translate PubMed's XML to Rust structures and therefore JSON.

## References
- [[Rust MCP Server Portfolio Project#DE-PUBMED-0002 Types]]
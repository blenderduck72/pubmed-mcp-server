# CLI

This package contains a simple CLI for making REST requests to PubMed's API.

This is more for developers performing manual tests with the PubMed Client
defined in the `client` package.

## Running the CLI

To run the cli simply run the following in the terminal:

```shell
cargo run --package pubmed
```

This will output the following:

```shell
PubMed CLI for retrieving PubMedAbstracts

Usage: pubmed.exe <COMMAND>

Commands:
  search
  fetch
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The CLI can be built and made as a sharable binary via the:

```shell
cargo build --package pubmed --release
```

It can be installed via `cargo install`

## Commands

The CLI has the following commands for testing the `Client` package:

### Search

Details about the search command can be accessed via:

```shell
cargo run --package pubmed search --help
```

It should output the following:

```shell
Usage: pubmed.exe search [OPTIONS] --expression <EXPRESSION>

Options:
  -e, --expression <EXPRESSION>  ESSIE Syntax Query
  -o, --out <OUT>                Output Location (Optional)
  -h, --help                     Print help
```

#### Example Usage

```shell
cargo run --package pubmed search -e "cancer"
```

#### Parameters

The Search CLI accepts the following parameters:

- **expression**
- **out**

##### Expression

PubMed's API accepts a query written in an ESSIE Syntax, the CLI accepts this
expression as a string.

For more information on forming queries for PubMed Refer to their
[documentation](https://pubmed.ncbi.nlm.nih.gov/help/#how-do-i-search-pubmed).

As an example of an a query with an advanced expression:

```shell
cargo run --package pubmed search -e "cancer AND 2020/06/01[dp]"
```

##### Out

By default the CLI will output the content as a markdown text to the terminal.
This can be modified so that it is written to a file instead of the terminal,
by passing in a file path for the value of `out`.

### Fetch

Details about the fetch command can be viewed by running:

```shell
cargo run --package pubmed fetch --help
```

It should output the following:

```shell
Usage: pubmed.exe fetch [OPTIONS]

Options:
  -i, --ids <IDS>  List of Article IDs to fetch
  -o, --out <OUT>  Output Location (Otional)
  -h, --help       Print help
```

#### Example Usage

```shell
cargo run --package pubmed fetch -i 36415761,34316686
```

#### Parameters

The Fetch CLI accept the following parameters:

- **ids**
- **output**

##### Ids

The list of PubMed Article Ids to fetch.

##### Out

By default the CLI will output the content as a markdown text to the terminal.
This can be modified so that it is written to a file instead of the terminal,
by passing in a file path for the value of `out`.

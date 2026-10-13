use clap::{Parser, Subcommand};
use client::{Client, utils};
use console::Term;
use std::{io::Write, path::PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Parser)]
struct SearchArgs {
    #[arg(
        short, long
    )]
    expression: String,

    #[arg(
        short, long
    )]
    out: Option<PathBuf>,
}

#[derive(Parser)]
struct FetchArgs {
    #[arg(
        short, long
    )]
    ids: Vec<String>,

    #[arg(
        short, long
    )]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Search(SearchArgs),
    Fetch(FetchArgs),
}

#[derive(Parser)]
#[clap(
    about = "PubMed CLI for retrieving PubMedAbstracts",
    name = "pubmed",
    version = "0.0.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

async fn fetch(
    args: &FetchArgs,
    client: &Client,
    term: &mut Term,
) -> anyhow::Result<()> {
    let fetch_results = client
        .fetch(&args.ids)
        .await?;
    let output = utils::pub_med_article_set_to_markdown(fetch_results).await;

    match &args.out {
        Some(out) => {
            let mut file = tokio::fs::File::create(out).await?;
            file.write_all(output.as_bytes())
                .await?;
            file.flush()
                .await?;
        }
        None => {
            term.write_all(output.as_bytes())?;
        }
    }

    Ok(())
}

async fn search(
    args: &SearchArgs,
    client: &Client,
    term: &mut Term,
) -> anyhow::Result<()> {
    let search_results = client
        .search(
            &args.expression,
            None,
            Some(5),
        )
        .await?;
    let ids: Vec<String> = search_results
        .id_list
        .ids
        .into_iter()
        .filter_map(|id| id.value)
        .collect::<_>();
    let fetch_results = client
        .fetch(&ids)
        .await?;
    let output = utils::pub_med_article_set_to_markdown(fetch_results).await;

    match &args.out {
        Some(out) => {
            let mut file = tokio::fs::File::create(out).await?;
            file.write_all(output.as_bytes())
                .await?;
            file.flush()
                .await?;
        }
        None => {
            term.write_all(output.as_bytes())?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let client = Client::new(
        None, None, None,
    )?;
    let mut term = Term::stdout();
    term.clear_screen()?;

    term.write_line(&format!(
        "{}",
        console::style("PubMed CLI").magenta()
    ))?;
    term.write_line(&format!(
        "{}",
        console::style("---").cyan()
    ))?;

    match args.command {
        Command::Search(args) => {
            search(
                &args, &client, &mut term,
            )
            .await?;
        }
        Command::Fetch(args) => {
            fetch(
                &args, &client, &mut term,
            )
            .await?;
        }
    };

    Ok(())
}

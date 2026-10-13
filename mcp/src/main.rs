mod app;
mod errors;
mod mcp;
mod models;

use clap::Parser;
use client::Client;
use console::Term;
use mcp::Mcp;

const BIND_ADDRESS: &str = "127.0.0.1";

#[derive(Parser)]
struct Cli {
    #[clap(
        short,
        long,
        default_value = "5004"
    )]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let term = Term::stdout();
    let client = Client::new(
        None, None, None,
    )?;
    let mcp = Mcp::new(client);

    let cancellation_token = tokio_util::sync::CancellationToken::new();
    let app = app::build_app(
        mcp,
        Some(&cancellation_token),
    )
    .await;

    let address = format!(
        "{}:{}",
        &BIND_ADDRESS, &cli.port
    );
    let tcp_listener = tokio::net::TcpListener::bind(&address).await?;

    let start_message = format!(
        "🟢 | Running at: {}",
        console::style(&address).green()
    );
    term.write_line(&start_message)?;
    let term_clone = term.clone();

    axum::serve(
        tcp_listener,
        app,
    )
    .with_graceful_shutdown(
        async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed");

            term_clone
                .clear_last_lines(1)
                .expect("STDOUT Error");
            term_clone
                .write_line("🟡 | Shutting Down")
                .expect("STDOUT Error");

            cancellation_token.cancel();

            term_clone
                .clear_last_lines(1)
                .expect("STDOUT Error");
            term_clone
                .write_line("🟠 | Shutting Down")
                .expect("STDOUT Error");
            term_clone
                .clear_last_lines(1)
                .expect("STDOUT Error");
        },
    )
    .await?;

    term.clear_screen()?;

    Ok(())
}

use anyhow::Result;
use clap::Clap;

mod cli;
mod runner;

use cli::Opts;
use runner::run_test;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    run_test(opts.url, opts.requests, opts.connections).await?;
    Ok(())
}

use anyhow::Result;
use clap::Clap;

mod cli;
mod runner;

use cli::Opts;
use runner::run_test;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    run_test(opts.uri, opts.requests, opts.workers).await?;
    Ok(())
}

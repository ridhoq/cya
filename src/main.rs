use anyhow::Result;
use clap::Clap;

mod cli;
mod runner;
mod worker;

use cli::Opts;
use runner::run_test;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    run_test(opts.uri, opts.requests, opts.workers)
}

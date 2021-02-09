use clap::Clap;

mod cli;

use cli::Opts;

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

use clap::Clap;
use reqwest::Url;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!())]
/// A HTTP load testing utility
pub struct Opts {
    /// Number of requests to send to the HTTP URL under test
    #[clap(short, long, default_value = "1000")]
    pub requests: i32,
    /// Number of workers. Controls concurrency of requests. Not implemented yet
    #[clap(short, long, default_value = "4")]
    pub workers: i32,
    /// HTTP URL under test
    #[clap(required = true, name = "URL", parse(try_from_str))]
    pub url: Url,
}

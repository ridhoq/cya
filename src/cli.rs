use clap::Clap;
use reqwest::Url;
use reqwest::Method;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!())]
/// A HTTP load testing utility
pub struct Opts {
    /// Number of requests to send to the HTTP URL under test
    #[clap(short, long, default_value = "1000")]
    pub requests: i32,
    /// Maximum number of concurrent connections
    #[clap(short, long, default_value = "32")]
    pub connections: i32,
    #[clap(short, long, name = "Method", default_value = Method::GET, parse(try_from_str))]
    pub method: Method,
    /// HTTP URL under test
    #[clap(required = true, name = "URL", parse(try_from_str))]
    pub url: Url,
}

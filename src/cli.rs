use clap::Clap;
use hyper::Uri;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!())]
/// A HTTP load testing utility
pub struct Opts {
    /// Number of requests to send to the HTTP URI under test
    #[clap(short, long, default_value = "1000")]
    pub requests: i32,
    /// Number of workers. Controls concurrency of requests. Not implemented yet
    #[clap(short, long, default_value = "4")]
    pub workers: i32,
    /// HTTP URI under test
    #[clap(required = true, name = "URI", parse(try_from_str))]
    pub uri: Uri,
}

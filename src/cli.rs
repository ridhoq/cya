use clap::Clap;
use hyper::Uri;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!())]
/// A HTTP load testing utility
pub struct Opts {
    /// Number of requests to send to the HTTP URI under test
    #[clap(short, long, default_value = "1000")]
    requests: i32,
    /// Number of workers. Controls parallelization of requests
    #[clap(short, long, default_value = "4")]
    workers: i32,
    /// HTTP URI to test
    #[clap(required = true, name = "URI", parse(try_from_str))]
    uri: Uri,
}

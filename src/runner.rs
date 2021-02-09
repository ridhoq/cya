use anyhow::Result;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;

type CyaHttpsConnector = HttpsConnector<HttpConnector>;
type CyaClient = Client<CyaHttpsConnector, Body>;

/// Runs the load test
pub async fn run_test(uri: Uri, requests: i32, _workers: i32) -> Result<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<CyaHttpsConnector, Body>(https);
    for _ in 0..requests {
        request(&client, &uri).await?;
    }
    Ok(())
}

pub async fn request(client: &CyaClient, uri: &Uri) -> Result<()> {
    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .header(
            "User-Agent",
            format!("{} {}", clap::crate_name!(), clap::crate_version!()),
        )
        // TODO: couldn't figure out how to not pass a body with the Request::builder
        // TODO: pass in a real body when doing POST/PUT/etc
        .body(Body::from(""))
        .unwrap();
    client.request(req).await?;
    Ok(())
}

use anyhow::Result;
use reqwest::{Client, Request, Method, Url, Response};

fn get_user_agent() -> String {
    format!("{} {}", clap::crate_name!(), clap::crate_version!())
}

/// Runs the load test
pub async fn run_test(url: Url, requests: i32, _workers: i32) -> Result<()> {
    let client = Client::builder()
        .user_agent(get_user_agent())
        .build()?;
    for _ in 0..requests {
        request(&client, &url).await?;
    }
    Ok(())
}

pub async fn request(client: &Client, url: &Url) -> Result<()> {
    let req = client.request(Method::GET, &url.to_string());
    req.send().await?;
    Ok(())
}

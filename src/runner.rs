use anyhow::Result;
use reqwest::{Client, Method, Url};
use std::option::Option::Some;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;

fn get_user_agent() -> String {
    format!("{}/{}", clap::crate_name!(), clap::crate_version!())
}

/// Runs the load test
pub async fn run_test(url_arg: Url, requests: i32, connections: i32) -> Result<()> {
    let url = Arc::new(url_arg);
    let succeeded = Arc::new(Mutex::new(0));

    let (sender, mut reciever) = mpsc::channel(connections as usize);
    task::spawn(async move {
        for _ in 0..requests {
            let url = Arc::clone(&url);
            let handle = task::spawn(async move {
                let url = Arc::clone(&url);
                let client = Client::builder().user_agent(get_user_agent()).build()?;
                client.request(Method::GET, url.as_str()).send().await
            });
            sender.send(handle).await;
        }
    });

    let receive_handle = task::spawn(async move {
        while let Some(res) = reciever.recv().await {
            let thing = res.await.expect("oops");
            let real_res = thing.expect("oof");
            if real_res.status().is_success() {
                let mut succeeded_lock = succeeded.lock().expect("Could not acquire lock");
                *succeeded_lock += 1;
            }
        }
        let succeeded_lock = succeeded.lock().unwrap();
        println!("succeeded requests: {}", *succeeded_lock);
    });

    receive_handle.await?;
    Ok(())
}

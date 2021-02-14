use anyhow::Result;
use reqwest::{Client, Method, Url};
use std::option::Option::Some;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use uuid::Uuid;

fn get_user_agent(id: Uuid) -> String {
    format!("{}/{}/{}", clap::crate_name!(), clap::crate_version!(), id)
}

/// Runs the load test
pub async fn run_test(url: Url, requests: i32, connections: i32) -> Result<()> {
    let correlation_id = Uuid::new_v4();
    let url = Arc::new(url);
    let succeeded = Arc::new(Mutex::new(0));
    let failed = Arc::new(Mutex::new(0));
    
    println!("correlation id: {}", correlation_id);

    let (sender, mut reciever) = mpsc::channel(connections as usize);
    task::spawn(async move {
        for _ in 0..requests {
            let url = Arc::clone(&url);
            let handle = task::spawn(async move {
                let url = Arc::clone(&url);
                let client = Client::builder().user_agent(get_user_agent(correlation_id)).build()?;
                client.request(Method::GET, url.as_str()).send().await
            });
            sender.send(handle).await.expect("Failed to send to receiver");
        }
    });

    let receive_handle = task::spawn(async move {
        while let Some(handle) = reciever.recv().await {
            let result = handle.await.expect("oops");
            match result {
                Ok(res) => {
                    if res.status().is_success(){
                        let mut succeeded = succeeded.lock().expect("Could not acquire lock");
                        *succeeded += 1;
                    } 
                },
                Err(_) => {
                    let mut failed = failed.lock().expect("Could not acquire lock");
                    *failed += 1;
                }
            } 
        }
        
        let succeeded = succeeded.lock().unwrap();
        let failed = failed.lock().unwrap();
        println!("succeeded requests: {}", *succeeded);
        println!("failed requests: {}", *failed);
    });

    receive_handle.await?;
    Ok(())
}

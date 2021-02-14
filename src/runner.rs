use anyhow::Result;
use reqwest::{Client, Method, Url};
use serde::Serialize;
use std::collections::HashMap;
use std::option::Option::Some;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CyaResult {
    succeeded: i32,
    failed: i32,
    response_codes: HashMap<String, i32>,
    correlation_id: String,
}

fn get_user_agent(id: Uuid) -> String {
    format!("{}/{}/{}", clap::crate_name!(), clap::crate_version!(), id)
}

/// Runs the load test
///
/// The load tester generates load by spawning tasks that make the HTTP request.
/// It creates a channel that is bounded by the connections argument which effectively controls the
/// number of concurrent connections at any given time.
///
/// The sender task starts a task that makes the HTTP request and sends the handle of that task to
/// the receiver task
///
/// The receiver task awaits the result of the HTTP request tasks and counts the number of successes
/// and failures. In the future, this will do further aggregation of response status codes and durations.
pub async fn run_test(url: Url, requests: i32, connections: i32) -> Result<()> {
    let correlation_id = Uuid::new_v4();
    let client = Arc::new(
        Client::builder()
            .user_agent(get_user_agent(correlation_id))
            .build()?,
    );
    let url = Arc::new(url);

    println!("correlation id: {}", correlation_id);

    let (sender, mut reciever) = mpsc::channel(connections as usize);

    task::spawn(async move {
        for _ in 0..requests {
            let client = Arc::clone(&client);
            let url = Arc::clone(&url);
            let handle = task::spawn(async move {
                let client = Arc::clone(&client);
                let url = Arc::clone(&url);
                client.request(Method::GET, url.as_str()).send().await
            });
            sender
                .send(handle)
                .await
                .expect("Failed to send to receiver");
        }
    });

    let receive_handle = task::spawn(async move {
        let mut succeeded = 0;
        let mut failed = 0;
        let mut response_codes = HashMap::new();

        while let Some(handle) = reciever.recv().await {
            let result = handle.await.expect("oops");
            match result {
                Ok(res) => {
                    let response_code_count =
                        response_codes.entry(res.status().to_string()).or_insert(0);
                    *response_code_count += 1;
                    if res.status().is_success() {
                        succeeded += 1;
                    }
                }
                Err(err) => {
                    failed += 1;
                }
            }
        }

        let result = CyaResult {
            succeeded,
            failed,
            response_codes,
            correlation_id: correlation_id.to_string(),
        };
        let serialized = serde_json::to_string_pretty(&result).unwrap();
        println!("{}", serialized);
    });

    receive_handle.await?;
    Ok(())
}

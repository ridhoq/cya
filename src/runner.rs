use anyhow::Result;
use average::{Estimate, Max, Min, Quantile};
use reqwest::{Client, Method, Url, Response, Error};
use serde::Serialize;
use std::collections::HashMap;
use std::option::Option::Some;
use std::sync::Arc;
use std::time::{Instant, Duration};
use tokio::sync::mpsc;
use tokio::task;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CyaHistogram {
    min: f64,
    p50: f64,
    p95: f64,
    p99: f64,
    max: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CyaResult {
    succeeded: i32,
    failed: i32,
    histogram: CyaHistogram,
    response_codes: HashMap<String, i32>,
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

    let now = Instant::now();

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
        let mut response_results = vec![];

        while let Some(handle) = reciever.recv().await {
            let result = handle.await.expect("oops");
            let duration = now.elapsed();
            response_results.push((result, duration));
        }

        let result = get_cya_result(response_results);
        let serialized = serde_json::to_string_pretty(&result).unwrap();
        println!("{}", serialized);
    });

    receive_handle.await?;
    Ok(())
}

fn get_cya_result(response_results: Vec<(Result<Response, Error>, Duration)>) -> CyaResult {
    let mut succeeded = 0;
    let mut failed = 0;
    let mut response_codes = HashMap::new();
    let mut durations = vec![];
    
    for (result, duration) in response_results {
        durations.push(duration.as_secs_f64());
        
        match result {
            Ok(res) => {
                let response_code_count =
                    response_codes.entry(res.status().to_string()).or_insert(0);
                *response_code_count += 1;
                if res.status().is_success() {
                    succeeded += 1;
                }
            }
            Err(_) => {
                failed += 1;
            }
        }
    }
    
    let histogram = get_cya_histogram(durations);
    
    CyaResult {
        succeeded,
        failed,
        histogram,
        response_codes,
    }
}

fn get_cya_histogram(durations: Vec<f64>) -> CyaHistogram {
    let mut min = Min::new();
    let mut p50 = Quantile::new(0.5);
    let mut p95 = Quantile::new(0.95);
    let mut p99 = Quantile::new(0.99);
    let mut max = Max::new();
    
    for duration in durations {
        min.add(duration);
        p50.add(duration);
        p95.add(duration);
        p99.add(duration);
        max.add(duration);
    }
    
    CyaHistogram {
        min: min.min(),
        p50: p50.quantile(),
        p95: p95.quantile(),
        p99: p99.quantile(),
        max: max.max(),
    }
}

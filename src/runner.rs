use anyhow::Result;
use average::{Estimate, Max, Min, Quantile};
use reqwest::{Client, Error, Method, Response, Url};
use serde::Serialize;
use std::collections::HashMap;
use std::option::Option::Some;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CyaFailureReasons {
    body: u32,
    builder: u32,
    connect: u32,
    decode: u32,
    redirect: u32,
    status: u32,
    timeout: u32,
}

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
    failure_reasons: CyaFailureReasons,
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
                let now = Instant::now();
                let result = client.request(Method::GET, url.as_str()).send().await;
                let duration = now.elapsed();
                (result, duration)
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
            response_results.push(result);
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
    let mut errors = vec![];

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
            Err(err) => {
                failed += 1;
                if let Some(status) = err.status() {
                    let response_code_count = response_codes.entry(status.to_string()).or_insert(0);
                    *response_code_count += 1;
                }
                errors.push(err);
            }
        }
    }

    let histogram = get_cya_histogram(durations);
    let failure_reasons = get_cya_failure_reasons(errors);

    CyaResult {
        succeeded,
        failed,
        histogram,
        response_codes,
        failure_reasons,
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

fn get_cya_failure_reasons(errors: Vec<Error>) -> CyaFailureReasons {
    let mut body = 0;
    let mut builder = 0;
    let mut connect = 0;
    let mut decode = 0;
    let mut redirect = 0;
    let mut status = 0;
    let mut timeout = 0;
    
    for err in errors {
        if err.is_body() {
            body += 1
        }
        if err.is_builder() {
            builder += 1
        }
        if err.is_connect() {
            connect += 1
        }
        if err.is_decode() {
            decode += 1
        }
        if err.is_redirect() {
            redirect += 1
        }
        if err.is_status() {
            status += 1
        }
        if err.is_timeout() {
            timeout += 1
        }
    }
    
    CyaFailureReasons {
        body,
        builder,
        connect,
        decode,
        redirect,
        status,
        timeout,
    }
}

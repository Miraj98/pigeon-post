use postman_collection_serializer::serialize;
use runtime::runner;
use futures::{stream, StreamExt};
use std::{env, time::Instant};
use colored::Colorize;
use serde_json::Value;
use reqwest::{Client, header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE}};


#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let buffer_param = &args[2];
    let client = Client::new();

    let start = Instant::now();
    let inputs = serialize(file_path.as_str()).unwrap();
    let buffer_param = match buffer_param.parse::<usize>() {
        Ok(val) => val,
        Err(_) => inputs.len(),
    };
    let serialization_in = start.elapsed().as_millis();

    println!("Collection serialization completed in ...{serialization_in}ms\n\n");

    
    let resp = runner(inputs).await;
    let lanes = 1;
    let _json_stream = stream::iter(resp)
        .map(|v| {
            async move {
                if v.is_ok() {
                    let runner_res = v.ok().take().unwrap();
                    println!("{:?}", runner_res);
                }
            }
        })
        .buffered(lanes)
        .collect::<Vec<_>>()
        .await;
    
    Ok(())
}

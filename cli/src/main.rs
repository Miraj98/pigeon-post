use postman_collection_serializer::serialize;
use runtime::runner;
use futures::{stream, StreamExt};
use std::{env, time::Instant};
use colored::Colorize;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let start = Instant::now();

    let inputs = serialize(file_path.as_str()).unwrap();

    let serialization_in = start.elapsed().as_millis();
    println!("Collection serialization completed in ...{serialization_in}ms\n\n");

    let resp = runner(inputs).await;
    let lanes = 1;
    let _json_stream = stream::iter(resp)
        .map(|v| {
            async move {
                if v.response.is_ok() {
                    let resp = v.response.ok().take().unwrap();
                    if resp.status().is_success() {
                        let start = Instant::now();
                        let status = resp.status().to_string();
                        let url = resp.url().to_string();
                        let json = resp.json::<Value>().await;
                        let time_taken = start.elapsed().as_millis();

                        println!("{} ...{}", "Success".green().bold(), status);
                        println!("URL: {url}");
                        println!("  ...request completed in {}ms", v.time_taken);
                        println!("  ...response serialization completed in {}ms\n", time_taken);
                    } else {
                        println!("{} ...{}", "Failed".red().bold(), resp.status().as_str());
                        println!("URL: {}", resp.url().to_string());
                        println!("  ...request completed in {}ms\n", v.time_taken);
                    }
                }
            }
        })
        .buffered(lanes)
        .collect::<Vec<_>>()
        .await;
    Ok(())
}

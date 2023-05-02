use postman_collection_serializer::serialize;
use runtime::Runtime;
use futures::{stream, StreamExt};
use serde_json::Value;
use std::env;
use colored::Colorize;


#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let inputs = serialize(file_path.as_str()).unwrap();

    let pigeon_runtime = Runtime::new();
    let resp = pigeon_runtime.send_multiple(inputs).await;
    
    let _json_stream = stream::iter(resp)
        .then(|v| {
            async move {
                if v.response.is_ok() {
                    let r = v.response.ok().take().unwrap();
                    let req_str = format!("[GET] {}", r.url().to_string());
                    let result_str = match r.status().is_success() {
                        true => format!("{} {} [{}ms]", String::from("Success").green(), r.status().to_string(), v.time_taken.to_string().bold()),
                        false => format!("{} {} [{}ms]", String::from("Failed").red(), r.status().to_string(), v.time_taken.to_string().bold())
                    };

                    println!("{result_str}");
                    println!("{}\n", req_str.dimmed());

                    let _ = r.json::<Value>().await;
                }
            }
        })
        .collect::<Vec<_>>()
        .await;
    
    Ok(())
}

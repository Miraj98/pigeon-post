use postman_collection_serializer::serialize;
use runtime::runner;
use futures::{stream, StreamExt};
use std::env;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];

    let inputs = serialize(file_path.as_str()).unwrap();
    let resp = runner(inputs).await;
    let _json_stream = stream::iter(resp)
        .map(|v| {
            async move {
                let val = v.unwrap().text().await;
                if val.is_ok() {
                    println!("is_ok = {}", val.unwrap());
                } else {
                    println!("{:?}", val.err());
                }
            }
        })
        .buffer_unordered(4)
        .collect::<Vec<_>>()
        .await;
    Ok(())
}

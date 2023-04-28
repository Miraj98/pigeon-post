use postman_collection_serializer::serialize;
use runtime::runner;
use futures::{stream, StreamExt};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let inputs = serialize("/Users/mirajshah/Downloads/college-cms.json").unwrap();
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

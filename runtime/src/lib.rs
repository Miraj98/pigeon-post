mod core;

use std::str::FromStr;

use futures::{stream, StreamExt};
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Error, Response};
pub use crate::core::{RequestInput, Header};

pub async fn runner(input: Vec<RequestInput>) -> Vec<Result<Response, Error>> {
    let client = reqwest::Client::new();
    let resp_stream = stream::iter(input).map(|mut r| {
        let mut builder = match r.method.as_str() {
            "GET" => client.get(r.url),
            "POST" => client.post(r.url),
            _ => unimplemented!()
        };
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(r.headers.len());
        while r.headers.len() > 0 {
            let h = r.headers.pop().unwrap();
            let h_name = HeaderName::from_str(h.key.as_str()).unwrap();
            let h_value = HeaderValue::from_str(h.key.as_str()).unwrap();
            headers.insert(h_name, h_value);
        } 
        builder = builder.headers(headers);

        if r.body.is_some() {
            builder = builder.body(r.body.take().unwrap());
        }

        async move {
            builder.send().await
        }
    }).buffer_unordered(4);

    let results = resp_stream.collect::<Vec<_>>().await;
    results
}


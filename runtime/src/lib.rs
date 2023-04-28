mod core;

use std::str::FromStr;

use futures::{stream, StreamExt};
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE}, Error, Response};
use tokio::time::Instant;
pub use crate::core::{RequestInput, Header};

pub struct RunnerResponse {
    pub time_taken: u128,
    pub response: Result<Response, Error>
}

pub async fn runner(input: Vec<RequestInput>) -> Vec<RunnerResponse> {
    let client = reqwest::Client::new();
    let lanes = input.len();
    let resp_stream = stream::iter(input).map(|mut r| {
        let mut builder = match r.method.as_str() {
            "GET" => client.get(r.url),
            "POST" => client.post(r.url),
            _ => unimplemented!()
        };
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(r.headers.len() + 1);
        for h in r.headers.iter_mut() {
            let h_name = HeaderName::from_str(h.key.as_str()).unwrap();
            let h_value = HeaderValue::from_str(h.key.as_str()).unwrap();
            headers.insert(h_name, h_value);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        builder = builder.headers(headers);

        if r.body.is_some() {
            builder = builder.body(r.body.take().unwrap());
        }

        async move {
            let now = Instant::now();
            let response = builder.send().await;
            let time_taken = now.elapsed().as_millis();
            RunnerResponse { response, time_taken }
        }
    }).buffered(lanes);

    let results = resp_stream.collect::<Vec<_>>().await;
    results
}


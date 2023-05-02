mod structs;

use std::time::Instant;
use futures::{ stream, StreamExt };
use reqwest::{ header::{ HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE }, Error };
use serde_json::Value;

pub use crate::structs::{ RequestInput, Header };


#[derive(Debug)]
pub struct RunnerResponse {
    pub time_taken: u128,
    pub response: Result<Value, Error>
}

pub async fn runner(input: Vec<RequestInput>) -> Vec<Result<RunnerResponse, ()>> {
    let client = reqwest::Client::new();
    let lanes = input.len();
    let resp_stream = stream::iter(input).map(|mut r| {
        let mut req = match r.method.as_str() {
           "GET" => client.get(&r.url),
            "POST" => client.post(&r.url),
            _ => unimplemented!("[{}] request type is not yet implemented", r.method)
        };
        
        let mut headers = HeaderMap::new();
        for h in r.headers.iter() {
            let h_name = h
                .key
                .parse::<HeaderName>()
                .map_err(|err| eprintln!("[ERROR] Unable to parse {} as a HeaderName: {err}", h.key))
                .unwrap();
            let h_value = h
                .value
                .parse::<HeaderValue>()
                .map_err(|err| eprintln!("[ERROR] Unable to parse {} as a HeaderValue: {err}", h.value))
                .unwrap();
            headers.insert(h_name, h_value);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        if r.body.is_some() {
            req = req.body(r.body.take().unwrap());
        }

        async move {
            let now = Instant::now();
            let response = req
                .send()
                .await
                .map_err(|err| eprintln!("[ERROR] {} {} failed: {err}", r.method, r.url))?
                .json::<Value>()
                .await;
            let time_taken = now.elapsed().as_millis();
            Ok::<_, ()>(RunnerResponse { response, time_taken })
        }
    }).buffer_unordered(lanes);

    let results = resp_stream.collect::<Vec<_>>().await;
    results
}


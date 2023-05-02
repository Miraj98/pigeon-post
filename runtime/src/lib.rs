mod structs;

use std::time::Instant;
use futures::{ stream, StreamExt };
use reqwest::header::{ HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE };
pub use crate::structs::{ Runtime, RequestInput, Response, Header };

impl Runtime {
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
    }

    fn make_req(&self, input: &mut RequestInput) -> reqwest::RequestBuilder {
        let mut req = match input.method.as_str() {
           "GET" => self.client.get(&input.url),
            "POST" => self.client.post(&input.url),
            _ => unimplemented!("[{}] request type is not yet implemented", input.method)
        };
        let mut headers = HeaderMap::new();
        for h in input.headers.iter() {
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
        if input.body.is_some() {
            req = req.body(input.body.take().unwrap());
        }

        req

    }

    pub async fn send(&self, mut input: RequestInput) -> Response {
        let req = self.make_req(&mut input);
        let now = Instant::now();
        let response = req.send().await;
        let time_taken = now.elapsed().as_millis();
        Response { response, time_taken }
    }

    pub async fn send_multiple(&self, input: Vec<RequestInput>) -> Vec<Response> {
        let lanes = input.len();
        let results = stream::iter(input)
            .map(|mut r| {
                let req = self.make_req(&mut r);
                async move {
                    let now = Instant::now();
                    let response = req.send().await;
                    let time_taken = now.elapsed().as_millis();
                    Response { response, time_taken }
                }
            })
            .buffer_unordered(lanes)
            .collect::<Vec<_>>()
            .await;
        
        results
    }
}


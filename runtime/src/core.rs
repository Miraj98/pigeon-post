use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestInput {
    pub method: String,
    pub url: String,
    pub headers: Vec<Header>,
    pub body: Option<Vec<u8>>
}

#[derive(Deserialize, Debug)]
pub struct Header {
    pub key: String,
    pub value: String
}

impl RequestInput {
    pub fn new(method: String, url: String, headers: Vec<Header>, body: Option<Vec<u8>>) -> Self {
        RequestInput { method, url, headers, body }
    }
}

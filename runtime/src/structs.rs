use serde::Deserialize;

pub struct Runtime {
    pub(crate) client: reqwest::Client
}

#[derive(Debug)]
pub struct Response {
    pub time_taken: u128,
    pub response: Result<reqwest::Response, reqwest::Error>,
}

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

    pub fn builder() -> RequestInputBuilder {
        RequestInputBuilder {
            method: String::from("GET"),
            url: String::from(""),
            headers: Vec::with_capacity(4),
            body: None
        }
    }
}

pub enum HttpMethod {
    GET,
    POST
}

pub struct RequestInputBuilder {
    method: String,
    url: String,
    headers: Vec<Header>,
    body: Option<Vec<u8>>
}

impl RequestInputBuilder {
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = match method {
            HttpMethod::GET => String::from("GET"),
            HttpMethod::POST => String::from("POST")
        };
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.push(Header { key, value });
        self
    }

    pub fn headers(mut self, headers: Vec<(String, String)>) -> Self {
        headers.iter().for_each(|(key, value)| {
            self.headers.push(Header { key: key.to_string(), value: value.to_string() })
        });
        self
    }

    pub fn body_u8(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn body_json(mut self, body: &impl serde::Serialize) -> Self {
        let buf = serde_json::to_vec(body).map_err(|err| eprintln!("[ERROR] Cannot convert passed body to Vec<u8>:{err}")).unwrap();
        self.body = Some(buf);
        self
    }

    pub fn body_str(mut self, body: &str) -> Self {
        let buf = body.as_bytes().to_vec();
        self.body = Some(buf);
        self
    }

    pub fn build(self) -> RequestInput {
        RequestInput::new(self.method, self.url, self.headers, self.body)
    }
}



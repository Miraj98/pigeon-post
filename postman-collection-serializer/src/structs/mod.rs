mod utils;
mod url;
mod body;

use serde::Deserialize;
pub use utils::sanitize_wildcard;
pub use url::{Url, UrlQuery};
pub use body::Body;


#[derive(Debug, Deserialize)]
pub struct Collection {
    pub info: Info,
    pub item: Vec<Item>,
    pub variable: Option<Vec<KeyValueObject>>,
    pub auth: Option<Auth>
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub schema: String
}


#[derive(Debug, Deserialize)]
pub struct Auth {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub bearer: Option<Vec<KeyValueObject>>,
    pub apikey: Option<Vec<KeyValueObject>>
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub request: RequestObject
}

#[derive(Debug, Deserialize)]
pub struct RequestObject {
    pub method: String,
    pub url: Url,
    pub header: Vec<KeyValueObject>,
    pub body: Option<Body>
}


#[derive(Debug, Deserialize)]
pub struct KeyValueObject {
    pub key: String,
    pub value: String
}


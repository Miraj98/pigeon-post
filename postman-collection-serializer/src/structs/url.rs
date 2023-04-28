use serde::Deserialize;
use std::{collections::HashMap, fmt::Write};
use crate::sanitize_wildcard;


#[derive(Debug, Deserialize)]
pub struct Url {
    pub raw: String,
    pub host: Vec<String>,
    pub path: Vec<String>,
    pub query: Option<Vec<UrlQuery>>
}

impl Url {
    pub fn url_string(&self, var_map: &HashMap<String, String>) -> String {
        let mut ret_str = String::with_capacity(self.raw.len());
        let host = sanitize_wildcard(self.host[0].clone(), var_map);
        let path: Vec<String> = self.path.iter().map(|p| sanitize_wildcard(p.clone(), var_map)).collect();
        let query: Vec<String>;
        if self.query.is_none() {
            query = Vec::<String>::new();
        } else {
            query = self
                .query
                .as_ref()
                .unwrap()
                .iter()
                .filter(|q| q.disabled.is_none() || !q.disabled.unwrap())
                .map(|q| {
                    let key = sanitize_wildcard(q.key.clone(), var_map);
                    let value = sanitize_wildcard(q.value.clone(), var_map);
                    return format!("{key}={value}");
                }).collect();
        }
        
        write!(&mut ret_str, "{host}").unwrap();

        if path.len() > 0 {
            if !host.ends_with('/') {
                ret_str.write_char('/').unwrap();
            }
            write!(&mut ret_str, "{}", path.join("/")).unwrap();
        }

        if query.len() > 0 {
            ret_str.write_char('?').unwrap();
            write!(&mut ret_str, "{}", query.join("&")).unwrap();
        }

        ret_str
    }
}

#[derive(Debug, Deserialize)]
pub struct UrlQuery {
    pub key: String,
    pub value: String,
    pub disabled: Option<bool>
}


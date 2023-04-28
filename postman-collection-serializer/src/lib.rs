mod structs;

use std::{fs, collections::HashMap};
use runtime::{RequestInput, Header};
use structs::{Collection, sanitize_wildcard};


pub fn serialize(input: &str) -> Result<Vec<RequestInput>, String> { 
    let json_str = fs::read_to_string(input).expect("Invalid file path");
    let mut collection: Collection = serde_json::from_str(json_str.as_str()).expect("Invalid collection schema");

    let split_url: Vec<&str> = collection.info.schema.split("/").collect();
    assert_eq!(split_url[5], "v2.1.0", "Only schema version v2.1.0 is supported right now ({} was passed instead)", split_url[5]);

    let mut var_map: HashMap<String, String>;

    if let Some(mut variable) = collection.variable.take() {
        var_map = HashMap::with_capacity(variable.len());
        while variable.len() > 0 {
            let kv = variable.pop().unwrap();
            var_map.insert(kv.key, kv.value);
        }
    } else {
        var_map = HashMap::with_capacity(1);
    }

    let mut ret: Vec<RequestInput> = Vec::with_capacity(collection.item.len());

    for item in collection.item.iter_mut() {
        let mut headers: Vec<Header> = Vec::with_capacity(item.request.header.len() + 1);
        while item.request.header.len() > 0 {
            let h = item.request.header.pop().unwrap();
            headers.push(Header {
                key: h.key,
                value: h.value
            });
        }

        if collection.auth.is_some() {
            if collection.auth.as_ref().unwrap().auth_type == "bearer" {
                headers.push(
                    Header {
                        key: "Authorization".to_string(),
                        value: sanitize_wildcard(collection.auth.as_ref().unwrap().bearer.as_ref().unwrap()[0].value.clone(), &var_map)
                    }
                );
            } else if collection.auth.as_ref().unwrap().auth_type == "apikey" {
                let key: String;
                let value: String;
                let kv_arr = collection.auth.as_ref().unwrap().apikey.as_ref().unwrap();
                if kv_arr[0].key.as_str() == "key" {
                    key = sanitize_wildcard(kv_arr[0].value.clone(), &var_map);
                    value = sanitize_wildcard(kv_arr[1].value.clone(), &var_map);
                } else {
                    key = sanitize_wildcard(kv_arr[1].value.clone(), &var_map);
                    value = sanitize_wildcard(kv_arr[0].value.clone(), &var_map);
                }
                headers.push(Header { key, value });
            }
        }

        let mut body: Option<Vec<u8>> = None;
        if item.request.body.is_some() {
            body = Some(item.request.body.as_ref().unwrap().to_buffer());
        }

        ret.push(RequestInput::new(item.request.method.clone(), item.request.url.url_string(&var_map), headers, body));
    }

    eprintln!("{:#?}", ret);

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_works() {
        let _ = serialize("test-data/test-postman.json").unwrap();
    }

}

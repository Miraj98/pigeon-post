use std::collections::HashMap;

pub fn sanitize_wildcard(val: String, var_map: &HashMap<String, String>) -> String {
    if val.starts_with("{{") && val.ends_with("}}") {
        let mut val_chars = val.chars();
        val_chars.next();
        val_chars.next();
        val_chars.next_back();
        val_chars.next_back();
        let key = val_chars.as_str().to_owned();
        let value = var_map.get(&key).expect(format!("Variable {{{}}} is not present in the collection variables", key.as_str()).as_str());
        return value.clone();
    } else {
        return val;
    }
}

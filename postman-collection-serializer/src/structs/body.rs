use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Body {
    pub mode: String,
    pub raw: Option<String>
}

impl Body {
    pub fn to_buffer(&self) -> Vec<u8> {
        match self.mode.as_str() {
            "raw" => self.raw.as_ref().unwrap().as_bytes().to_owned(),
            _ => unimplemented!(),
        }
    }
}


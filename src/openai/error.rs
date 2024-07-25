use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: InnerError
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnerError {
    pub message: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub param: Option<String>,
    pub code: String,
}
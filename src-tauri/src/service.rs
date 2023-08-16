use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub enum ResponseType {
    SUCCESS,
    FAILURE,
    CANCELED,
}

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub result: ResponseType,
    pub response: Option<T>,
    pub error: Option<String>,
}

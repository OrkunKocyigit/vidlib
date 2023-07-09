use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ResponseType {
    SUCCESS,
    FAILURE
}

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub result: ResponseType,
    pub response: Option<T>,
    pub error: Option<String>
}
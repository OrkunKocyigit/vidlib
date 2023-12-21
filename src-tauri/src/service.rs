use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub enum ResponseType {
    Success,
    Failure,
    Canceled,
}

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub result: ResponseType,
    pub response: Option<T>,
    pub error: Option<String>,
}

pub(crate) fn wrap_success<T>(response: T) -> Response<T> {
    Response {
        result: ResponseType::Success,
        response: Some(response),
        error: None,
    }
}
pub(crate) fn wrap_failure<T>(error: String) -> Response<T> {
    Response {
        result: ResponseType::Failure,
        response: None,
        error: Some(error),
    }
}
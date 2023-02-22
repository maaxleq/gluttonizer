use axum::http::StatusCode;
use serde_json::Error;

pub struct ProgramError {
    pub msg: &'static str
}

impl From<Error> for ProgramError {
    fn from(_: Error) -> Self {
        ProgramError { msg: "Could not deserialize data" }
    }
}

impl From<std::io::Error> for ProgramError {
    fn from(_: std::io::Error) -> Self {
        ProgramError { msg: "Could not open file" }
    }
}

pub struct RequestError {
    pub msg: String,
    pub status: StatusCode
}

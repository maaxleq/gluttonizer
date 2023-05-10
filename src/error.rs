use std::{sync::PoisonError, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse};
use serde_json::Error;

#[derive(Debug)]
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

impl<T> From<PoisonError<T>> for ProgramError {
    fn from(_: PoisonError<T>) -> Self {
        ProgramError { msg: "Lock held by a failed thread" }
    }
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ProgramError {}

pub struct RequestError {
    pub msg: String,
    pub status: StatusCode
}

impl IntoResponse for RequestError {
    fn into_response(self) -> axum::response::Response {
        (self.status, "").into_response()
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl RequestError {
    pub fn with_msg(mut self, msg: &str) -> RequestError {
        self.msg = String::from(msg);
        self
    }

    pub fn internal_server_error() -> RequestError {
        RequestError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: String::from("")
        }
    }

    pub fn service_unavailable() -> RequestError {
        RequestError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            msg: String::from("")
        }
    }

    pub fn not_found() -> RequestError {
        RequestError {
            status: StatusCode::NOT_FOUND,
            msg: String::from("")
        }
    }

    pub fn bad_request() -> RequestError {
        RequestError {
            status: StatusCode::BAD_REQUEST,
            msg: String::from("")
        }
    }

    pub fn forbidden() -> RequestError {
        RequestError {
            status: StatusCode::FORBIDDEN,
            msg: String::from("")
        }
    }

    pub fn im_a_teapot() -> RequestError {
        RequestError {
            status: StatusCode::IM_A_TEAPOT,
            msg: String::from("")
        }
    }
}

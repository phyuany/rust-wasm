use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

// 没有用到NotFound，rust会发生警告，但是不希望发生警告，所以加上#[allow(dead_code)]
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum MyError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for MyError {}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::TeraError(msg) => {
                println!("Error in rendering the template: {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::ActixError(_msg) | MyError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND
        }
    }
    fn error_response(&self) -> HttpResponse {
        let resp = MyErrorResponse {
            error_message: self.error_response(),
        };
        HttpResponse::build(self.status_code()).json(resp)
    }
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

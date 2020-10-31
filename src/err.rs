use std::{error::Error, fmt};
use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct ResponseError {
    code: usize,
    msg: String,
}

impl ResponseError {
    pub fn new(code: usize, message: String) -> ResponseError {
        ResponseError { code, msg: message }
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Internal Server response Error!code:{},msg:{}",
            self.code, self.msg
        )
    }
}

impl Error for ResponseError {}

#[test]
fn test_response_error() {
    let err = ResponseError::new(500, String::from("recovery"));
    println!("{}", err)
}

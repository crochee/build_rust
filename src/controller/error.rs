use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ResponseError {
    code: usize,
    msg: String,
}

impl ResponseError {
    fn new(code: usize, msg: &str) -> ResponseError {
        ResponseError {
            code,
            msg: msg.into_string(),
        }
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Internal Server response Error!code:{},msg:{})", self.code, self.msg)
    }
}

impl Error for ResponseError {}

#[test]
fn test_response_error() {
    let err = ResponseError::new(500, "recovery");
    println!("{}", err)
}
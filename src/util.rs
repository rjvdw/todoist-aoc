use std::{error, fmt};

pub type BoxedResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct HttpError {
    pub status: String,
    pub message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Http Error {} - {}", self.status, self.message)
    }
}

impl error::Error for HttpError {}

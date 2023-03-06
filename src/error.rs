use std::fmt;
use std::io::Error;
#[derive(Debug)]
pub enum EzyErr {
    TypeMismatch(String),
    ValueNotFound(String),
    IoError(Error),
}
impl fmt::Display for EzyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EzyErr::TypeMismatch(s) => write!(f, "Type mismatch: {}", s),
            EzyErr::ValueNotFound(s) => write!(f, "Value not found: {}", s),
            EzyErr::IoError(s) => write!(f, "{}", s)
        }
    }
}

impl std::error::Error for EzyErr {
    fn description(&self) -> &str {
        match self {
            EzyErr::TypeMismatch(_) => "Type mismatch error",
            EzyErr::ValueNotFound(_) => "Value not found error",
            EzyErr::IoError(_) => "Got IO Exception",
        }
    }
}

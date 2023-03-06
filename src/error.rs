use std::fmt;

#[derive(Debug)]
pub enum EzyErr {
    TypeMismatch(String),
    ValueNotFound(String),
}
impl fmt::Display for EzyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EzyErr::TypeMismatch(s) => write!(f, "Type mismatch: {}", s),
            EzyErr::ValueNotFound(s) => write!(f, "Value not found: {}", s),
        }
    }
}

impl std::error::Error for EzyErr {
    fn description(&self) -> &str {
        match self {
            EzyErr::TypeMismatch(_) => "Type mismatch error",
            EzyErr::ValueNotFound(_) => "Value not found error",
        }
    }
}

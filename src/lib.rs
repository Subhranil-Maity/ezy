use core::fmt;
use std::{collections::HashMap, path::Path};

pub enum EzyValue {
    SignedInteger8(i8),
    SignedInteger16(i16),
    SignedInteger32(i32),
    SignedInteger64(i64),
    SignedInteger128(i128),
    UnSignedInteger8(u8),
    UnSignedInteger16(u16),
    UnSignedInteger32(u32),
    UnSignedInteger64(u64),
    UnSignedInteger128(u128),
    Float32(f32),
    Float64(f64),
    Bool(bool),
}
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

pub fn parse_ezy(path: &Path) -> EzyKeyValuePair {
    unimplemented!("parse_ezy");
}

pub struct EzyKeyValuePair {
    pairs: HashMap<String, EzyValue>,
    version: String,
    last_updated: i64,
}

impl EzyKeyValuePair {
    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_value(&self, key: &str) -> Result<&EzyValue, EzyErr> {
        match self.pairs.get(key) {
            Some(value) => Ok(value),
            None => Err(EzyErr::ValueNotFound(format!("Key not found: '{}'", key))),
        }
    }

    pub fn get_signed_integer_8(&self, key: &str) -> Result<&i8, EzyErr> {
        match self.get_value(key)? {
            EzyValue::SignedInteger8(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected i8 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_signed_integer_16(&self, key: &str) -> Result<&i16, EzyErr> {
        match self.get_value(key)? {
            EzyValue::SignedInteger16(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected i16 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_signed_integer_32(&self, key: &str) -> Result<&i32, EzyErr> {
        match self.get_value(key)? {
            EzyValue::SignedInteger32(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected i32 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_signed_integer_64(&self, key: &str) -> Result<&i64, EzyErr> {
        match self.get_value(key)? {
            EzyValue::SignedInteger64(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected i64 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_signed_integer_128(&self, key: &str) -> Result<&i128, EzyErr> {
        match self.get_value(key)? {
            EzyValue::SignedInteger128(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected i128 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_unsigned_integer_8(&self, key: &str) -> Result<&u8, EzyErr> {
        match self.get_value(key)? {
            EzyValue::UnSignedInteger8(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected u8 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_unsigned_integer_16(&self, key: &str) -> Result<&u16, EzyErr> {
        match self.get_value(key)? {
            EzyValue::UnSignedInteger16(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected u16 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_unsigned_integer_32(&self, key: &str) -> Result<&u32, EzyErr> {
        match self.get_value(key)? {
            EzyValue::UnSignedInteger32(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected u32 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_unsigned_integer_64(&self, key: &str) -> Result<&u64, EzyErr> {
        match self.get_value(key)? {
            EzyValue::UnSignedInteger64(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected u64 for key '{}'",
                key
            ))),
        }
    }

    pub fn get_unsigned_integer_128(&self, key: &str) -> Result<&u128, EzyErr> {
        match self.get_value(key)? {
            EzyValue::UnSignedInteger128(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected u128 for key '{}'",
                key
            ))),
        }
    }
    pub fn get_float_32(&self, key: &str) -> Result<&f32, EzyErr> {
        match self.get_value(key)? {
            EzyValue::Float32(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected f32 for key '{}'",
                key
            ))),
        }
    }
    pub fn get_float_64(&self, key: &str) -> Result<&f64, EzyErr> {
        match self.get_value(key)? {
            EzyValue::Float64(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected f64 for key '{}'",
                key
            ))),
        }
    }
    pub fn get_bool(&self, key: &str) -> Result<&bool, EzyErr> {
        match self.get_value(key)? {
            EzyValue::Bool(value) => Ok(value),
            _ => Err(EzyErr::TypeMismatch(format!(
                "Expected bool for key '{}'",
                key
            ))),
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

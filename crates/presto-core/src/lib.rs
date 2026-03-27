use std::fmt::{Display, Formatter};

pub type PrestoResult<T> = Result<T, PrestoError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrestoError {
    InvalidArgument(&'static str),
    InvalidType {
        expected: &'static str,
        actual: &'static str,
    },
    Domain(&'static str),
    Overflow(&'static str),
    Unsupported(&'static str),
    Internal(String),
}

impl Display for PrestoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument(msg) => write!(f, "invalid argument: {msg}"),
            Self::InvalidType { expected, actual } => {
                write!(f, "invalid type: expected {expected}, got {actual}")
            }
            Self::Domain(msg) => write!(f, "domain error: {msg}"),
            Self::Overflow(msg) => write!(f, "overflow error: {msg}"),
            Self::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for PrestoError {}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValueRef<'a> {
    Null,
    Integer(i64),
    Real(f64),
    Text(&'a str),
    Blob(&'a [u8]),
}

impl<'a> SqlValueRef<'a> {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Integer(_) => "integer",
            Self::Real(_) => "real",
            Self::Text(_) => "text",
            Self::Blob(_) => "blob",
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_i64(&self) -> PrestoResult<i64> {
        match self {
            Self::Integer(v) => Ok(*v),
            Self::Real(v) => {
                if !v.is_finite() {
                    return Err(PrestoError::InvalidArgument(
                        "cannot cast NaN/inf to integer",
                    ));
                }
                if *v < i64::MIN as f64 || *v > i64::MAX as f64 {
                    return Err(PrestoError::Overflow("real out of i64 range"));
                }
                Ok(*v as i64)
            }
            _ => Err(PrestoError::InvalidType {
                expected: "integer",
                actual: self.type_name(),
            }),
        }
    }

    pub fn as_f64(&self) -> PrestoResult<f64> {
        match self {
            Self::Integer(v) => Ok(*v as f64),
            Self::Real(v) => Ok(*v),
            _ => Err(PrestoError::InvalidType {
                expected: "real",
                actual: self.type_name(),
            }),
        }
    }

    pub fn as_text(&self) -> PrestoResult<&'a str> {
        match self {
            Self::Text(v) => Ok(*v),
            _ => Err(PrestoError::InvalidType {
                expected: "text",
                actual: self.type_name(),
            }),
        }
    }

    pub fn as_blob(&self) -> PrestoResult<&'a [u8]> {
        match self {
            Self::Blob(v) => Ok(*v),
            _ => Err(PrestoError::InvalidType {
                expected: "blob",
                actual: self.type_name(),
            }),
        }
    }

    pub fn into_owned(self) -> SqlValue {
        match self {
            Self::Null => SqlValue::Null,
            Self::Integer(v) => SqlValue::Integer(v),
            Self::Real(v) => SqlValue::Real(v),
            Self::Text(v) => SqlValue::Text(v.to_owned()),
            Self::Blob(v) => SqlValue::Blob(v.to_vec()),
        }
    }
}

impl SqlValue {
    pub fn as_ref(&self) -> SqlValueRef<'_> {
        match self {
            Self::Null => SqlValueRef::Null,
            Self::Integer(v) => SqlValueRef::Integer(*v),
            Self::Real(v) => SqlValueRef::Real(*v),
            Self::Text(v) => SqlValueRef::Text(v.as_str()),
            Self::Blob(v) => SqlValueRef::Blob(v.as_slice()),
        }
    }
}

pub fn result_null() -> SqlValue {
    SqlValue::Null
}

pub fn result_int(v: i64) -> SqlValue {
    SqlValue::Integer(v)
}

pub fn result_real(v: f64) -> SqlValue {
    SqlValue::Real(v)
}

pub fn result_text(v: impl Into<String>) -> SqlValue {
    SqlValue::Text(v.into())
}

pub fn result_blob(v: impl Into<Vec<u8>>) -> SqlValue {
    SqlValue::Blob(v.into())
}

#[derive(Debug, Default)]
pub struct FunctionRegistry {
    scalar_functions: Vec<&'static str>,
    window_functions: Vec<&'static str>,
}

impl FunctionRegistry {
    pub fn register_scalar(&mut self, name: &'static str) {
        self.scalar_functions.push(name);
    }

    pub fn register_window(&mut self, name: &'static str) {
        self.window_functions.push(name);
    }

    pub fn scalar_functions(&self) -> &[&'static str] {
        &self.scalar_functions
    }

    pub fn window_functions(&self) -> &[&'static str] {
        &self.window_functions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_and_real_conversions_work() {
        assert_eq!(SqlValueRef::Integer(7).as_i64().unwrap(), 7);
        assert_eq!(SqlValueRef::Real(7.9).as_i64().unwrap(), 7);
        assert_eq!(SqlValueRef::Integer(7).as_f64().unwrap(), 7.0);
    }

    #[test]
    fn conversion_rejects_non_numeric_type() {
        let err = SqlValueRef::Text("abc").as_i64().unwrap_err();
        assert!(matches!(
            err,
            PrestoError::InvalidType {
                expected: "integer",
                actual: "text"
            }
        ));
    }

    #[test]
    fn conversion_rejects_infinite_real() {
        let err = SqlValueRef::Real(f64::INFINITY).as_i64().unwrap_err();
        assert!(matches!(err, PrestoError::InvalidArgument(_)));
    }

    #[test]
    fn text_and_blob_helpers_work() {
        assert_eq!(SqlValueRef::Text("hello").as_text().unwrap(), "hello");
        assert_eq!(SqlValueRef::Blob(&[1, 2, 3]).as_blob().unwrap(), &[1, 2, 3]);
    }

    #[test]
    fn result_builders_produce_expected_values() {
        assert_eq!(result_null(), SqlValue::Null);
        assert_eq!(result_int(5), SqlValue::Integer(5));
        assert_eq!(result_real(2.5), SqlValue::Real(2.5));
        assert_eq!(result_text("x"), SqlValue::Text("x".into()));
        assert_eq!(result_blob(vec![9, 8]), SqlValue::Blob(vec![9, 8]));
    }

    #[test]
    fn registry_tracks_scalar_and_window_names() {
        let mut registry = FunctionRegistry::default();
        registry.register_scalar("abs");
        registry.register_window("row_number");

        assert_eq!(registry.scalar_functions(), &["abs"]);
        assert_eq!(registry.window_functions(), &["row_number"]);
    }
}

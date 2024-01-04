use std::str::FromStr;

use mtiny_core::{http::HeaderName, BoxError, Request};

pub fn header<T>(request: &Request, name: HeaderName) -> Result<T, ExtractHeaderError>
where
    T: FromStr,
    T::Err: Into<BoxError>,
{
    if let Some(value) = request.headers().get(&name) {
        match value.to_str() {
            Ok(s) => s
                .parse::<T>()
                .map_err(|e| ExtractHeaderError::InvalidHeader {
                    name,
                    source: e.into(),
                }),
            Err(e) => Err(ExtractHeaderError::InvalidHeader {
                name,
                source: e.into(),
            }),
        }
    } else {
        Err(ExtractHeaderError::MissingHeader { name })
    }
}
#[derive(Debug)]
pub enum ExtractHeaderError {
    MissingHeader { name: HeaderName },
    InvalidHeader { name: HeaderName, source: BoxError },
}

impl core::fmt::Display for ExtractHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractHeaderError::InvalidHeader { name, source: _ } => {
                write!(f, "invalid request header `{}`", name)
            }
            ExtractHeaderError::MissingHeader { name } => {
                write!(f, "missing request header `{}`", name)
            }
        }
    }
}

impl std::error::Error for ExtractHeaderError {}

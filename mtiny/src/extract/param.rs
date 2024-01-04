use std::{collections::HashMap, str::FromStr};

use mtiny_core::{BoxError, Request};
use mtiny_router::Params;

pub fn params(request: &Request) -> Option<&HashMap<String, String>> {
    crate::extract::extension(request).map(|params: &Params| params.get_ref())
}
pub fn param_raw<'a>(request: &'a Request, name: &str) -> Option<&'a str> {
    params(request).and_then(|params| params.get(name).map(|v| v.as_str()))
}
pub fn param<T>(request: &Request, name: &str) -> Result<T, ExtractParamError>
where
    T: FromStr,
    T::Err: Into<BoxError>,
{
    param_raw(request, name).map_or_else(
        || Err(ExtractParamError::MissingParam { name: name.into() }),
        |param| {
            param
                .parse::<T>()
                .map_err(|e| ExtractParamError::InvalidParam {
                    name: name.into(),
                    source: e.into(),
                })
        },
    )
}
#[derive(Debug)]
pub enum ExtractParamError {
    MissingParam { name: String },
    InvalidParam { name: String, source: BoxError },
}

impl std::fmt::Display for ExtractParamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractParamError::MissingParam { name } => {
                write!(f, "missing route param {} ", name)
            }
            ExtractParamError::InvalidParam { name, source: _ } => {
                write!(f, "invalid route param {} ", name)
            }
        }
    }
}

impl std::error::Error for ExtractParamError {}

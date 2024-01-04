use mtiny_core::Request;
use serde::Deserialize;

pub fn query<'de, T>(request: &'de Request) -> Result<T, ExtractQueryError>
where
    T: Deserialize<'de>,
{
    let query = request.uri().query().unwrap_or_default();
    serde_urlencoded::from_str(query).map_err(ExtractQueryError)
}

#[derive(Debug)]
pub struct ExtractQueryError(pub serde::de::value::Error);

impl std::fmt::Display for ExtractQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to deserialize query string ({})", self.0)
    }
}

impl std::error::Error for ExtractQueryError {}

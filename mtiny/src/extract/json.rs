use mtiny_core::{http::header, BoxError, Request};
use serde::de::DeserializeOwned;

pub async fn json<T>(request: &mut Request) -> Result<T, ExtractJsonError>
where
    T: DeserializeOwned,
{
    if !is_json_content_type(request) {
        return Err(ExtractJsonError::UnsupportedContentType);
    }

    let bytes = crate::extract::bytes(request)
        .await
        .map_err(|e| ExtractJsonError::FailedReadBody(e.into()))?;
    serde_json::from_slice(&bytes).map_err(ExtractJsonError::FailedToDeserialize)
}
fn is_json_content_type(request: &Request) -> bool {
    let content_type = if let Some(content_type) = request.headers().get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };
    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };
    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().filter(|name| *name == "json").is_some());
    is_json_content_type
}

#[derive(Debug)]
pub enum ExtractJsonError {
    UnsupportedContentType,
    FailedReadBody(BoxError),
    FailedToDeserialize(serde_json::Error),
}

impl core::fmt::Display for ExtractJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractJsonError::UnsupportedContentType => f.write_str("snsupported content type"),
            ExtractJsonError::FailedReadBody(e) => {
                write!(f, "failed to read body ({})", e)
            }
            ExtractJsonError::FailedToDeserialize(e) => {
                write!(f, "failed to deseerialize ({})", e)
            }
        }
    }
}

impl std::error::Error for ExtractJsonError {}

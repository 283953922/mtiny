use mtiny_core::{
    body::{Body, BodyExt}, http::header, http::HeaderValue, response::IntoResponse, response::Response,
    BoxError,
};

pub fn html<B>(b: B)->Html<B>{
    Html(b)
}

#[derive(Debug, Clone, Copy)]
pub struct Html<B>(pub B);

impl<B> IntoResponse for Html<B>
where
    B: Body + 'static,
    B::Error: Into<BoxError>,
{
    fn into_response(self) -> Response {
        let mut res = Response::new(self.0.boxed());
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
        );
        res
    }
}

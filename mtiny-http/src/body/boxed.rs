use std::pin::Pin;

use super::body::Body;
use super::ext::BodyExt;

pub struct BoxBody {
    inner: Pin<Box<dyn Body<Error = Box<dyn std::error::Error>>>>,
}

impl BoxBody {
    pub fn new<B>(body: B) -> Self
    where
        B: Body + 'static,
        B::Error: Into<Box<dyn std::error::Error>>,
    {
        Self {
            inner: Box::pin(body.map_err(Into::into)),
        }
    }
}

impl Body for BoxBody {
    type Error = Box<dyn std::error::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<bytes::Bytes, Self::Error>>> {
        self.inner.as_mut().poll_next(cx)
    }
    fn size_hint(&self) -> super::size_hint::SizeHint {
        self.inner.size_hint()
    }
}

impl Default for BoxBody {
    fn default() -> Self {
        BoxBody::new(())
    }
}
impl std::fmt::Debug for BoxBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxBody").finish()
    }
}

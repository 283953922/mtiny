use std::{pin::Pin, task::Poll};

use pin_project_lite::pin_project;

use super::body::Body;

pin_project! {
    #[derive(Clone, Copy)]
    pub struct MapErr<B,F>{
        #[pin]
        inner: B,
        f: F,
    }
}

impl<B, F> MapErr<B, F> {
    pub(crate) fn new(inner: B, f: F) -> Self {
        Self { inner, f }
    }

    pub fn get_ref(&self) -> &B {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut B {
        &mut self.inner
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B> {
        self.project().inner
    }

    pub fn into_inner(self) -> B {
        self.inner
    }
}

impl<B, F, E> Body for MapErr<B, F>
where
    B: Body,
    F: FnMut(B::Error) -> E,
{
    type Error = E;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<bytes::Bytes, Self::Error>>> {
        let project = self.project();
        match project.inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(data))) => Poll::Ready(Some(Ok(data))),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err((project.f)(err)))),
        }
    }
    fn size_hint(&self) -> super::size_hint::SizeHint {
        self.inner.size_hint()
    }
}

impl<B, F> std::fmt::Debug for MapErr<B, F>
where
    B: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapErr")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

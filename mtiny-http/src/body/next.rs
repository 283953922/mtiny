use std::{future::Future, pin::Pin};

use bytes::Bytes;

use super::body::Body;

#[must_use = "futures don't do anything unless polled"]
#[derive(Debug)]
pub struct Next<'a, B: ?Sized>(pub(crate) &'a mut B);

impl<'a, B> Future for Next<'a, B>
where
    B: Body + Unpin + ?Sized,
{
    type Output = Option<Result<Bytes, B::Error>>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

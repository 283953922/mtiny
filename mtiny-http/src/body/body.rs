use std::{
    borrow::Cow,
    convert::Infallible,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;

use crate::{Request, Response};

use super::size_hint::SizeHint;

pub trait Body {
    type Error;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>>;
    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }
}

impl<B> Body for Request<B>
where
    B: Body,
{
    type Error = B::Error;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        unsafe { self.map_unchecked_mut(Request::body_mut).poll_next(cx) }
    }
    fn size_hint(&self) -> SizeHint {
        self.body().size_hint()
    }
}

impl<B> Body for Response<B>
where
    B: Body,
{
    type Error = B::Error;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        unsafe { self.map_unchecked_mut(Response::body_mut).poll_next(cx) }
    }
    fn size_hint(&self) -> SizeHint {
        self.body().size_hint()
    }
}

impl<B> Body for &mut B
where
    B: Body + Unpin + ?Sized,
{
    type Error = B::Error;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Pin::new(&mut **self).poll_next(cx)
    }
    fn size_hint(&self) -> SizeHint {
        (**self).size_hint()
    }
}

impl<B> Body for Box<B>
where
    B: Body + Unpin + ?Sized,
{
    type Error = B::Error;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Pin::new(&mut **self).poll_next(cx)
    }
    fn size_hint(&self) -> SizeHint {
        (**self).size_hint()
    }
}

impl<P> Body for Pin<P>
where
    P: DerefMut + Unpin,
    P::Target: Body,
{
    type Error = <P::Target as Body>::Error;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        self.get_mut().as_mut().poll_next(cx)
    }
    fn size_hint(&self) -> SizeHint {
        (**self).size_hint()
    }
}

impl Body for () {
    type Error = Infallible;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Poll::Ready(None)
    }
    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(0)
    }
}

impl Body for &'static [u8] {
    type Error = Infallible;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Poll::Ready(Some(Ok(Bytes::from_static(std::mem::take(self.get_mut())))))
    }
    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for Vec<u8> {
    type Error = Infallible;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from(std::mem::take(self.get_mut())))))
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for Cow<'static, [u8]> {
    type Error = Infallible;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        match self.get_mut() {
            Cow::Borrowed(v) => Pin::new(v).poll_next(cx),
            Cow::Owned(v) => Pin::new(v).poll_next(cx),
        }
    }
    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for &'static str {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from_static(
                std::mem::take(self.get_mut()).as_bytes(),
            ))))
        }
    }
    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for String {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from(std::mem::take(self.get_mut())))))
        }
    }
}

impl Body for Cow<'static, str> {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        match self.get_mut() {
            Cow::Borrowed(v) => Pin::new(v).poll_next(cx),
            Cow::Owned(v) => Pin::new(v).poll_next(cx),
        }
    }
}

impl Body for Bytes {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(std::mem::take(self.get_mut()))))
        }
    }
    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

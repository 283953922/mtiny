use crate::Service;
use core::future::Future;
use futures_util::TryFutureExt;
opaque_future! {
    pub type AndThenFuture<Fut1, Fut2, F> = futures_util::future::AndThen<Fut1, Fut2, F>;
}

#[derive(Clone, Copy)]
pub struct AndThen<S, F> {
    inner: S,
    f: F,
}

impl<S, F> AndThen<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req, Res, Fut> Service<Req> for AndThen<S, F>
where
    S: Service<Req>,
    F: FnOnce(S::Response) -> Fut + Clone,
    Fut: Future<Output = Result<Res, S::Error>>,
{
    type Response = Res;
    type Error = S::Error;
    type Future = AndThenFuture<S::Future, Fut, F>;

    fn call(&self, request: Req) -> Self::Future {
        AndThenFuture::new(self.inner.call(request).and_then(self.f.clone()))
    }
}

impl<S, F> core::fmt::Debug for AndThen<S, F>
where
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AndThen")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

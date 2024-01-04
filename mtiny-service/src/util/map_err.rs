use futures_util::TryFutureExt;

use crate::Service;

opaque_future! {
pub type MapErrFuture<Fut,F> = futures_util::future::MapErr<Fut,F>;
}

#[derive(Clone, Copy)]
pub struct MapErr<S, F> {
    inner: S,
    f: F,
}

impl<S, F> MapErr<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req, Err> Service<Req> for MapErr<S, F>
where
    S: Service<Req>,
    F: FnOnce(S::Error) -> Err + Clone,
{
    type Response = S::Response;
    type Error = Err;
    type Future = MapErrFuture<S::Future, F>;
    fn call(&self, request: Req) -> Self::Future {
        MapErrFuture::new(self.inner.call(request).map_err(self.f.clone()))
    }
}

impl<S, F> core::fmt::Debug for MapErr<S, F>
where
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MapErr")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

use core::pin::Pin;

use core::future::Future;
use alloc::boxed::Box;

use crate::{Service, ServiceExt};

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

pub struct BoxService<Req, Res, Err> {
    inner: Box<dyn Service<Req, Response = Res, Error = Err, Future = BoxFuture<Result<Res, Err>>>>,
}

impl<Req, Res, Err> BoxService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + 'static,
        S::Future: 'static,
    {
        Self {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for BoxService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<Result<Res, Err>>;
    fn call(&self, request: Req) -> Self::Future {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> core::fmt::Debug for BoxService<Req, Res, Err> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BoxService").finish()
    }
}

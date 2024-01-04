use crate::Service;

pub struct MapRequest<S, F> {
    inner: S,
    f: F,
}

impl<S, F> MapRequest<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, R1, R2> Service<R1> for MapRequest<S, F>
where
    S: Service<R2>,
    F: Fn(R1) -> R2,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, request: R1) -> Self::Future {
        self.inner.call((self.f)(request))
    }
}

impl <S,F> core::fmt::Debug for MapRequest<S,F>
where
S:core::fmt::Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MapRequest")
        .field("inner", &self.inner)
        .field("f", &core::any::type_name::<F>())
        .finish()
    }
}

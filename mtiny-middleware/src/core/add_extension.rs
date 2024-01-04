use mtiny_core::service::{Service, Wrap};
use mtiny_core::Request;

#[derive(Clone, Copy)]
pub struct AddExtension<S, F> {
    inner: S,
    f: F,
}

pub fn add_extension<F>(f: F) -> AddExtensionWarp<F> {
    AddExtensionWarp::new(f)
}

impl<S, F> AddExtension<S, F> {
    fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}
impl<S, F, B, T> Service<Request<B>> for AddExtension<S, F>
where
    S: Service<Request<B>>,
    F: Fn() -> T,
    T: 'static,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = S::Future;

    fn call(&self, mut request: Request<B>) -> Self::Future {
        request.extensions_mut().insert((self.f)());
        self.inner.call(request)
    }
}

impl<S, F> core::fmt::Debug for AddExtension<S, F>
where
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddExtension")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct AddExtensionWarp<F> {
    f: F,
}

impl<F> AddExtensionWarp<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Wrap<S> for AddExtensionWarp<F> {
    type Service = AddExtension<S, F>;
    fn wrap(self, service: S) -> Self::Service {
        AddExtension::new(service, self.f)
    }
}

impl<F> core::fmt::Debug for AddExtensionWarp<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddExtensionWarp")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

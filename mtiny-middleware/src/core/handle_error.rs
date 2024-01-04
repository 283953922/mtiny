use std::convert::Infallible;

use futures_core::ready;
use futures_core::task::Poll;
use futures_core::Future;

use pin_project_lite::pin_project;

use mtiny_core::service::Service;
use mtiny_core::service::Wrap;

pub fn handle_error<F>(f: F) -> HandleErrorWap<F> {
    HandleErrorWap::new(f)
}
#[derive(Clone, Copy)]
pub struct HandleError<S, F> {
    inner: S,
    f: F,
}

impl<S, F> HandleError<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req> Service<Req> for HandleError<S, F>
where
    S: Service<Req>,
    F: FnOnce(S::Error) -> S::Response + Clone,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = HandleErrorFuture<S::Future, F>;
    fn call(&self, request: Req) -> Self::Future {
        HandleErrorFuture::Incomplete {
            fut: self.inner.call(request),
            f: self.f.clone(),
        }
    }
}

impl<S, F> core::fmt::Debug for HandleError<S, F>
where
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HandleError")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

pin_project! {
    #[project = HandleErrorFutureProj]
    #[project_replace = HandleErrorFutureProjReplace]
    pub enum HandleErrorFuture<Fut, F> {
        Incomplete {
            #[pin]
            fut: Fut,
            f: F,
        },
        Complete,
    }
}

impl<Fut, F, Res, Err> Future for HandleErrorFuture<Fut, F>
where
    Fut: Future<Output = Result<Res, Err>>,
    F: FnOnce(Err) -> Res,
{
    type Output = Result<Res, Infallible>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.as_mut().project() {
            HandleErrorFutureProj::Incomplete { fut, .. } => {
                let output = ready!(fut.poll(cx));
                match self.project_replace(HandleErrorFuture::Complete) {
                    HandleErrorFutureProjReplace::Incomplete { f, .. } => match output {
                        Ok(res) => Poll::Ready(Ok(res)),
                        Err(err) => Poll::Ready(Ok(f(err))),
                    },
                    HandleErrorFutureProjReplace::Complete => unreachable!(),
                }
            }
            HandleErrorFutureProj::Complete => {
                panic!("polled after completion")
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct HandleErrorWap<F> {
    f: F,
}

impl<F> HandleErrorWap<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, S> Wrap<S> for HandleErrorWap<F> {
    type Service = HandleError<S, F>;
    fn wrap(self, service: S) -> Self::Service {
        HandleError::new(service, self.f)
    }
}

impl<F> core::fmt::Debug for HandleErrorWap<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HandleErrorWap")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

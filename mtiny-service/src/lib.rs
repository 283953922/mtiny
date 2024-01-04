#![no_std]
#![forbid(unsafe_code)]
#[cfg(feature = "alloc")]
extern crate alloc;

use core::future::Future;

#[macro_use]
mod macros;

#[cfg(feature = "util")]
pub mod util;

#[cfg(feature = "util")]
pub use util::ServiceExt;

//pub use macros::*;

pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&self, request: Request) -> Self::Future;
}

impl<'a, S, Request> Service<Request> for &'a S
where
    S: Service<Request> + ?Sized + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

impl<'a, S, Request> Service<Request> for &'a mut S
where
    S: Service<Request> + ?Sized + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature="alloc")]
impl<S, Request> Service<Request> for alloc::boxed::Box<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature="alloc")]
impl<S, Request> Service<Request> for alloc::rc::Rc<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

pub trait Wrap<S> {
    type Service;
    fn wrap(self, service: S) -> Self::Service;
}

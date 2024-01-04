use crate::{Service, Wrap};

use super::{
    boxed::BoxService, map_err::MapErr, map_future::MapFuture, map_request::MapRequest,
    map_response::MapResponse, map_result::MapResult, then::Then, and_then::AndThen,
};
pub trait ServiceExt<Req>: Service<Req> {
    fn map_request<F>(self, f: F) -> MapRequest<Self, F>
    where
        Self: Sized,
    {
        MapRequest::new(self, f)
    }

    fn map_response<F>(self, f: F) -> MapResponse<Self, F>
    where
        Self: Sized,
    {
        MapResponse::new(self, f)
    }

    fn map_future<F>(self, f: F) -> MapFuture<Self, F>
    where
        Self: Sized,
    {
        MapFuture::new(self, f)
    }

    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
    {
        MapErr::new(self, f)
    }

    fn boxed(self) -> BoxService<Req, Self::Response, Self::Error>
    where
        Self: Sized + 'static,
        Self::Future: 'static,
    {
        BoxService::new(self)
    }
    fn with<T>(self, wrap: T) -> T::Service
    where
        Self: Sized,
        T: Wrap<Self>,
    {
        wrap.wrap(self)
    }

    fn map_result<F>(self, f: F) -> MapResult<Self, F>
    where
        Self: Sized,
    {
        MapResult::new(self, f)
    }

    fn then<F>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
    {
        Then::new(self, f)
    }
    fn and_then<F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
    {
        AndThen::new(self, f)
    }
}

impl<S, Req> ServiceExt<Req> for S where S: Service<Req> {}

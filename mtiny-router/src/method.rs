use mtiny_core::{
    http::Method,
    response::IntoResponse,
    service::{util::BoxService, Service, ServiceExt},
    BoxError, Request, Response,
};

use crate::{error::MethodNotAllowed, RouteFuture};

pub struct MethodRouter {
    get: Option<BoxService<Request, Response, BoxError>>,
    post: Option<BoxService<Request, Response, BoxError>>,
    put: Option<BoxService<Request, Response, BoxError>>,
    delete: Option<BoxService<Request, Response, BoxError>>,
    head: Option<BoxService<Request, Response, BoxError>>,
    patch: Option<BoxService<Request, Response, BoxError>>,
    trace: Option<BoxService<Request, Response, BoxError>>,
    options: Option<BoxService<Request, Response, BoxError>>,
}

macro_rules! method_router_impl_fn {
    ($method:ident) => {
        pub fn $method<S>(mut self, service: S) -> Self
        where
            S: Service<Request> + 'static,
            S::Response: IntoResponse,
            S::Error: Into<BoxError>,
        {
            self.$method = Some(Self::into_box_service(service));
            self
        }
    };
}

impl MethodRouter {
    fn new() -> Self {
        Self {
            get: None,
            post: None,
            put: None,
            delete: None,
            head: None,
            patch: None,
            trace: None,
            options: None,
        }
    }
    method_router_impl_fn!(get);
    method_router_impl_fn!(post);
    method_router_impl_fn!(put);
    method_router_impl_fn!(delete);
    method_router_impl_fn!(head);
    method_router_impl_fn!(patch);
    method_router_impl_fn!(trace);
    method_router_impl_fn!(options);

    fn into_box_service<S>(service: S) -> BoxService<Request, Response, BoxError>
    where
        S: Service<Request> + 'static,
        S::Response: IntoResponse,
        S::Error: Into<BoxError>,
    {
        service
            .map_response(IntoResponse::into_response)
            .map_err(Into::into)
            .boxed()
    }
}

impl Service<Request> for MethodRouter {
    type Response = Response;
    type Error = BoxError;
    type Future = RouteFuture;
    fn call(&self, request: Request) -> Self::Future {
        macro_rules! method_call {
            ($req:expr, $method:expr, $svc:expr) => {
                if $method == $req.method() {
                    if let Some(svc) = $svc {
                        return RouteFuture::Future {
                            fut: svc.call($req),
                        };
                    }
                }
            };
        }

        method_call!(request, Method::GET, &self.get);
        method_call!(request, Method::POST, &self.post);
        method_call!(request, Method::PUT, &self.put);
        method_call!(request, Method::DELETE, &self.delete);
        method_call!(request, Method::HEAD, &self.head);
        method_call!(request, Method::PATCH, &self.patch);
        method_call!(request, Method::PATCH, &self.trace);
        method_call!(request, Method::OPTIONS, &self.options);

        RouteFuture::Error {
            err: Some(MethodNotAllowed::new(request).into()),
        }
    }
}

macro_rules! route_method_impl_fn {
    ($method:ident) => {
        pub fn $method<S>(service: S) -> MethodRouter
        where
            S: Service<Request> + 'static,
            S::Response: IntoResponse,
            S::Error: Into<BoxError>,
        {
            MethodRouter::new().$method(service)
        }
    };
}

route_method_impl_fn!(get);
route_method_impl_fn!(post);
route_method_impl_fn!(put);
route_method_impl_fn!(delete);
route_method_impl_fn!(head);
route_method_impl_fn!(patch);
route_method_impl_fn!(trace);
route_method_impl_fn!(options);

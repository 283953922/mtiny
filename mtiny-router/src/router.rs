use core::future::Future;
use core::panic;
use core::task::Poll;
use std::collections::HashMap;

use matchit::Match;
use pin_project_lite::pin_project;

use mtiny_core::http::uri::{Parts, PathAndQuery, Uri};
use mtiny_core::response::IntoResponse;
use mtiny_core::service::{util::BoxFuture, util::BoxService, Service, ServiceExt};
use mtiny_core::{BoxError, Request, Response};

use crate::error::NotFound;

const PRIVATE_TAIL_PARAM: &'static str = "_private_xycy_tail_param";
enum Endpoint {
    Full(BoxService<Request, Response, BoxError>),
    Nest(BoxService<Request, Response, BoxError>),
}

pub struct Router {
    inner: matchit::Router<Endpoint>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            inner: matchit::Router::new(),
        }
    }

    fn add_route(mut self, path: String, endpoint: Endpoint) -> Self {
        if let Err(e) = self.inner.insert(path, endpoint) {
            panic!("{e}")
        }
        self
    }

    pub fn route<S>(self, path: &str, service: S) -> Self
    where
    S: Service<Request> + 'static,
    S::Response: IntoResponse,
    S::Error: Into<BoxError>,
    {
        if !path.starts_with('/') {
            panic!("Path must start with a `/`");
        }
        let path = if path.ends_with('*') {
            format!("{path}{PRIVATE_TAIL_PARAM}")
        } else {
            path.into()
        };
        self.add_route(path, Endpoint::Full(Self::into_box_service(service)))
    }


    pub fn nest<S>(self, path: &str, service: S) -> Self
    where
        S: Service<Request> + 'static,
        S::Response: IntoResponse,
        S::Error: Into<BoxError>,
    {
        if !path.starts_with('/') {
            panic!("Path must start with a `/`");
        }
        let path = if path.ends_with('/') {
            format!("{path}*{PRIVATE_TAIL_PARAM}")
        } else {
            format!("{path}/*{PRIVATE_TAIL_PARAM}")
        };
        self.add_route(path, Endpoint::Nest(Self::into_box_service(service)))
    }

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

impl core::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router").finish()
    }
}

impl Service<Request> for Router {
    type Response = Response;
    type Error = BoxError;
    type Future = RouteFuture;
    fn call(&self, mut request: Request) -> Self::Future {
        match self.inner.at(request.uri().path()) {
            Ok(Match { value, params }) => {
                let fut = match value {
                    Endpoint::Full(service) => {
                        let (params, _) = get_params(params);
                        insert_params(&mut request, params);
                        service.call(request)
                    }
                    Endpoint::Nest(service) => {
                        let (params, tail) = get_params(params);
                        insert_params(&mut request, params);
                        modify_path_and_query(&mut request, &tail.unwrap());
                        service.call(request)
                    }
                };
                RouteFuture::Future { fut }
            }
            Err(_) => RouteFuture::Error {
                err: Some(NotFound::new(request).into()),
            },
        }
    }
}

fn modify_path_and_query(request: &mut Request, path: &str) {
    let uri = request.uri_mut();

    let path_and_query = if let Some(query) = uri.query() {
        format!("{}?{}", path, query)
            .parse::<PathAndQuery>()
            .unwrap()
    } else {
        path.parse().unwrap()
    };

    let mut parts = Parts::default();

    parts.scheme = uri.scheme().cloned();
    parts.authority = uri.authority().cloned();
    parts.path_and_query = Some(path_and_query);

    *uri = Uri::from_parts(parts).unwrap();
}

#[derive(Debug, Clone)]
pub struct Params(HashMap<String, String>);

impl Params {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_ref(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }
}

fn get_params(params: matchit::Params) -> (Vec<(String, String)>, Option<String>) {
    let mut path = None;
    let params = {
        params
            .iter()
            .filter_map(|(k, v)| {
                if k == PRIVATE_TAIL_PARAM {
                    path = Some(v.to_owned());
                    None
                } else {
                    Some((k.to_owned(), v.to_owned()))
                }
            })
            .collect()
    };
    (params, path)
}

fn insert_params(request: &mut Request, captures: Vec<(String, String)>) {
    let extensions = request.extensions_mut();
    let params = if let Some(params) = extensions.get_mut::<Params>() {
        params
    } else {
        extensions.insert(Params::new());
        extensions.get_mut::<Params>().unwrap()
    };
    params.0.extend(captures)
}

pin_project! {
    #[project = RouteFutureProj]
pub enum  RouteFuture{
    Future {
        #[pin]
        fut: BoxFuture<Result<Response,BoxError>>,
    },
    Error {
        err: Option<BoxError>
    },
   }
}

impl Future for RouteFuture {
    type Output = Result<Response, BoxError>;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.project() {
            RouteFutureProj::Future { fut } => fut.poll(cx),
            RouteFutureProj::Error { err } => Poll::Ready(Err(err.take().expect("poll error"))),
        }
    }
}

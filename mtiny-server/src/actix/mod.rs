use std::convert::Infallible;
use std::net::SocketAddr;

use tokio::net::TcpStream;

use actix_http::HttpService;
use actix_service::IntoService;

use mtiny_core::response::IntoResponse;
use mtiny_core::service::Service;
use mtiny_core::BoxError;
use mtiny_core::Request;

mod compat;

pub struct Server<F> {
    factory: F,
    options: Result<ServerOptions, BoxError>,
}

pub struct ServerOptions {
    addr: Vec<SocketAddr>,
    workers: Option<usize>,
}

impl<F, S> Server<F>
where
    F: Fn() -> S + Clone + Send + 'static,
    S: Service<Request, Error = Infallible> + 'static,
    S::Response: IntoResponse,
    S::Future: 'static,
{
    pub fn new(factory: F) -> Self {
        Self {
            factory,
            options: Ok(ServerOptions {
                addr: vec![],
                workers: None,
            }),
        }
    }

    pub fn workers(mut self, num: usize) -> Self {
        self.options = self.options.and_then(|mut sp| {
            sp.workers = Some(num);
            Ok(sp)
        });
        self
    }

    pub fn bind<T>(mut self, addr: T) -> Self
    where
        T: Into<SocketAddr>,
    {
        self.options = self.options.and_then(|mut sp| {
            sp.addr.push(addr.into());
            Ok(sp)
        });
        self
    }

    pub async fn run(self) -> Result<(), BoxError> {
        let options = self.options?;
        let factory = self.factory;

        let factory = move || {
            let service = compat::into_actix_service(factory());
            let service = move |request: actix_http::Request| service.call(request);

            async move { Ok::<_, Infallible>(service.into_service()) }
        };

        let mut server = actix_server::Server::build();
        if let Some(workers) = options.workers {
            server = server.workers(workers);
        }

        server
            .bind("tiny", &options.addr[..], move || {
                HttpService::<TcpStream, _, _, _, _>::build()
                    .finish(factory.clone())
                    .tcp()
            })?
            .run()
            .await
            .map_err(From::from)
    }
}

impl<F> std::fmt::Debug for Server<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Server").finish()
    }
}

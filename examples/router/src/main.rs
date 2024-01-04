//! 路由示例。

mod config;
mod routers;
mod models;
mod services;

use std::convert::Infallible;
use std::future::Future;

use models::user::User;
use mtiny::extract;
use mtiny::http::StatusCode;
use mtiny::response::IntoResponse;
use mtiny::service::{Service, ServiceExt};
use mtiny::{middleware, route, service_fn, BoxError, Request, Response, Router, Server};
use routers::user_router::{UserRouter, UserQueryParam};
#[tokio::main]
async fn main() {
    Server::new(|| app())
        .bind(([127, 0, 0, 1], 8082))
        .run()
        .await
        .unwrap();
}

fn app() -> impl Service<
    Request,
    Response = Response,
    Error = Infallible,
    Future = impl Future<Output = Result<Response, Infallible>>,
> {
    
    Router::new()
    .route(
        "/user/add_user",
        route::post(service_fn(|req| {
            let user_param = extract::query::<User>(&req).unwrap();
            async  {
                let user =  UserRouter::new().add_user(user_param).await;
                Ok::<_, Infallible>(user)
            }
        })),
    )
        .route(
            "/user/get_user",
            route::get(service_fn(|req| {
                let user_param = extract::query::<UserQueryParam>(&req).unwrap();
                let user_id = user_param.user_id.clone();
                async  {
                    let user =  UserRouter::new().get_user(user_id).await;
                    Ok::<_, Infallible>(user)
                }
            })),
        ).route(
            "/user/del_user",
            route::get(service_fn(|req| {
                let user_param = extract::query::<UserQueryParam>(&req).unwrap();
                let user_id = user_param.user_id.clone();
                async  {
                    let result =  UserRouter::new().del_user(user_id).await;
                    Ok::<_, Infallible>(result)
                }
            })),
        )
        // 错误处理
        .with(middleware::handle_error(|err: BoxError| {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }))
}



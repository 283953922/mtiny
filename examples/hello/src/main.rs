use std::convert::Infallible;

use mtiny::{service_fn, Request, Server};
#[tokio::main]
async fn main() {
    Server::new(|| Box::new(service_fn(hello)))
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}

async fn hello(_req: Request) -> Result<String, Infallible> {
    Ok("hello world".to_string())
}

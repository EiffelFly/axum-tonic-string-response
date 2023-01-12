use std::net::SocketAddr;

use axum::Router;
use grpc_handler::GreeterServiceImpl;
use http::{header::CONTENT_TYPE, Request};
use hyper::Body;
use rest_handler::greeter_handler;
use tonic::transport::Server;
use tower::{steer::Steer, BoxError, ServiceExt};

mod grpc_handler;
mod rest_handler;
mod pb {
    include!("gen/mod.rs");
}

#[tokio::main]
async fn main() {
    let http = Router::new()
        .route("/", axum::routing::get(greeter_handler))
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .map_err(BoxError::from)
        .boxed_clone();

    let greeter_service =
        pb::helloworld::greeter_server::GreeterServer::new(GreeterServiceImpl::default());

    let grpc = Server::builder()
        .add_service(greeter_service)
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .map_err(BoxError::from)
        .boxed_clone();

    let http_grpc = Steer::new(vec![http, grpc], |req: &Request<Body>, _svcs: &[_]| {
        if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
            0
        } else {
            1
        }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8010));
    println!("Http Server started on 0.0.0.0:{:?}", 8010);

    axum::Server::bind(&addr)
        .serve(tower::make::Shared::new(http_grpc))
        .await
        .unwrap();
}

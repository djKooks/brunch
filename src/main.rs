use axum::{
    extract::Extension,
    http::{uri::Uri, Request, Response},
    routing::get,
    AddExtensionLayer, Router,
};
use hyper::{client::HttpConnector, Body};
use std::{convert::TryFrom, net::SocketAddr};

mod route_config;
use route_config::RouteConfig;

type Client = hyper::client::Client<HttpConnector, Body>;


#[tokio::main]
async fn main() {
    let client = Client::new();
    let route_config = RouteConfig::new("./test/configs/test_config.yml").unwrap();
    
    let mut app = Router::new();

    for (key, _) in route_config.route {
        app = app.route(key.as_ref(), get(proxy_handler));
    }

    app = app.layer(AddExtensionLayer::new(client));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn proxy_handler(Extension(client): Extension<Client>, mut req: Request<Body>) -> Response<Body> {
    // TODO: Make config parser as common logic, instead of calling separately
    let route_config = RouteConfig::new("./test/configs/test_config.yml").unwrap();
    let path_query: &str = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(req.uri().path());

    let reroute = route_config.route.get(path_query).unwrap();
    println!("From : {} -> To : {}", path_query, reroute);

    let uri = format!("http://127.0.0.1:9000{}", reroute);
    *req.uri_mut() = Uri::try_from(uri).unwrap();
    client.request(req).await.unwrap()
}

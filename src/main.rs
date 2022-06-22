mod parse;
mod html;
mod index;
mod about;
mod md;

use std::net::SocketAddr;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, get_service};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index::index))
        .route("/about", get(about::about))
        .nest("/favicon.ico",
              get_service(ServeDir::new("static/favicon.ico"))
                  .handle_error(|error: std::io::Error| async move {
                      (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                      )
            })
        )
        .nest( "/static",
               get_service(ServeDir::new("static"))
                   .handle_error(|error: std::io::Error| async move {
                       (
                           StatusCode::INTERNAL_SERVER_ERROR,
                           format!("Unhandled internal error: {}", error),
                       )
               }),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

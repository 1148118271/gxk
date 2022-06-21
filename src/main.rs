use std::net::SocketAddr;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use axum::routing::{get, get_service, post};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
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

async fn index() -> impl IntoResponse {
    let template = HelloTemplate {  };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

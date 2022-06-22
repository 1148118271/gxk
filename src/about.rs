use std::path::Path;
use axum::response::IntoResponse;
use askama::Template;
use crate::html::HtmlTemplate;
use crate::md;

pub async fn about() -> impl IntoResponse {
    let html = md::about();
    let template = AboutTemplate { info: html };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
   pub info: String
}

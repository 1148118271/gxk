use std::path::Path;
use axum::response::IntoResponse;
use askama::Template;
use axum::extract;
use crate::html::HtmlTemplate;
use crate::md;

pub async fn post(extract::Path(url): extract::Path<String>) -> impl IntoResponse {
    let html = md::post(url);
    let template = PostTemplate { info: html };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub info: String
}

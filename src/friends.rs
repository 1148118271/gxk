use axum::response::IntoResponse;
use askama::Template;
use crate::html::HtmlTemplate;
use crate::md;

pub async fn friends() -> impl IntoResponse {
    let html = md::friends();
    let template = FriendsTemplate { info: html };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "friends.html")]
pub struct FriendsTemplate {
    pub info: String
}

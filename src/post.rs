use axum::response::IntoResponse;
use askama::Template;
use axum::extract;
use crate::html::HtmlTemplate;
use crate::md;
use crate::parse;
use crate::parse::Post;

pub async fn post(extract::Path(url): extract::Path<String>) -> impl IntoResponse {
    let (html, title) = md::post(url);
    let template = PostTemplate { title, info: html };
    HtmlTemplate(template)
}

pub async fn post_all() -> impl IntoResponse {
    let mut posts = parse::Posts::init();
    posts.posts.reverse();
    let template = PostAllTemplate { posts: posts.posts };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub title: String,
    pub info: String
}


#[derive(Template)]
#[template(path = "postAll.html")]
pub struct PostAllTemplate {
    pub posts: Vec<Post>
}

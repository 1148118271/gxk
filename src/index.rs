use askama::Template;
use axum::response::IntoResponse;

use crate::parse::Post;
use crate::parse;
use crate::html::HtmlTemplate;

pub async fn index() -> impl IntoResponse {
    let mut posts = parse::Posts::init();

    posts.posts.reverse();

    let posts = if posts.posts.len() > 5 {
        posts.posts.truncate(5);
        posts.posts
    } else { posts.posts };

    let template = IndexTemplate {
        posts
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub posts: Vec<Post>
}

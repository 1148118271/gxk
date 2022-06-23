use std::path::{Path, PathBuf};
use crate::path;

fn get_html_str(path: PathBuf) -> String {
    let html = match markdown::file_to_html(path.as_path()) {
        Ok(v) => v,
        Err(e) => format!("Error: {}", e.to_string())
    };
    html
}


pub fn about() -> String {
    let path = path::filling("data/about.md");
    get_html_str(path)
}

pub fn friends() -> String {
    let path = path::filling("data/friends.md");
    get_html_str(path)
}

pub fn post(url: String) -> String {
    let p = format!("data/{}.md", url);
    let path = path::filling(p.as_str());
    get_html_str(path)
}

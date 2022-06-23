use std::path::Path;

fn get_html_str(path: &Path) -> String {
    let html = match markdown::file_to_html(path) {
        Ok(v) => v,
        Err(e) => format!("Error: {}", e.to_string())
    };
    html
}


pub fn about() -> String {
    let path = Path::new("data/about.md");
    get_html_str(path)
}

pub fn friends() -> String {
    let path = Path::new("data/friends.md");
    get_html_str(path)
}

pub fn post(url: String) -> String {
    let p = format!("data/{}.md", url);
    let path = Path::new(p.as_str());
    get_html_str(path)
}

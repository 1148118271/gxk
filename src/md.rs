use std::path::PathBuf;
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

pub fn post(url: String) -> (String, String) {
    let p = format!("data/{}.md", url);
    let path = path::filling(p.as_str());
    let hs = get_html_str(path);
    let vec = hs.split("\n").collect::<Vec<&str>>();
    let mut title = vec![];
    if vec.get(0).is_some() {
        let str = *(&vec[0]);
        let mut flag = false;
        for v in str.as_bytes() {
            if flag && *v == '<' as u8 {
                break
            }
            if flag {
                title.push(*v)
            }
            if *v == '>' as u8 {
                flag = true
            }
        }
    }

    (
        hs,
        unsafe {
            if title.is_empty() {
               String::from("Post")
            } else {
                String::from_utf8_unchecked(title)
            }
        }
    )
}

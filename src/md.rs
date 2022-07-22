use std::path::PathBuf;
use comrak::{ComrakOptions, markdown_to_html};
use crate::path;

fn get_html_str(path: PathBuf) -> String {
    let readme = std::fs::read_to_string(path.as_path()).unwrap();
    let mut comrak = ComrakOptions::default();
    comrak.extension.strikethrough = true;
    comrak.extension.table = true;
    comrak.extension.autolink = true;
    comrak.render.unsafe_ = true;
    markdown_to_html(&readme, &comrak)
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

use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use axum::response::IntoResponse;
use askama::Template;
use axum::{extract, Json};
use crate::about::AboutTemplate;
use crate::html::HtmlTemplate;
use crate::md;

pub async fn memo() -> impl IntoResponse {
    let vec = read_file().await;
    let template = MemoTemplate { memos: vec };
    HtmlTemplate(template)
}

pub async fn del(params: Json<HashMap<String, u32>>) -> impl IntoResponse {
    let mut vec = read_file().await;
    let line_num = params.get("lineNum");
    if line_num.is_none() {
        return r#"{"code": 500}"#
    }
    vec.remove(*(line_num.unwrap()) as usize - 1);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data/memo.d").unwrap();
    for line in &vec {
        file.write(format!("{}\r\n", line).as_bytes()).unwrap();
    }
    r#"{"code": 200}"#
}

async fn read_file() -> Vec<String> {
    let mut strings = vec![];
    if let Ok(file) = fs::File::open("data/memo.d") {
        let mut lines = BufReader::new(file).lines();
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            if let Ok(v) = line.unwrap() {
                strings.push(v);
            }
        }
    }
    strings
}


#[derive(Template)]
#[template(path = "memo.html")]
pub struct MemoTemplate {
    pub memos: Vec<String>
}
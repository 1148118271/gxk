use std::path::Path;

#[test] fn t1() {
    let path = Path::new("data/about.md");
    let string = markdown::file_to_html(path).unwrap();
    println!("{}", string)
}



pub fn about() -> String {
    let path = Path::new("data/about.md");
    let html = match markdown::file_to_html(path) {
        Ok(v) => v,
        Err(e) => format!("Error: {}", e.to_string())
    };
    html
}
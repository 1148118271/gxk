use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use crate::path;


#[derive(Deserialize, Debug)]
pub struct Posts {
    pub posts: Vec<Post>
}


impl Posts {
    pub fn init() -> Self {
        let path = path::filling("data/index.toml");
        if let Ok(mut file) = File::open(path) {
            let mut str_val = String::new();
            if let Ok(_) = file.read_to_string(&mut str_val) {
                if let Ok(toml) = toml::from_str(&str_val) {
                    return toml
                }
            }
        }
        Posts {
            posts: vec![]
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub date: String,
    pub url: String,
    pub title: String
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.date, self.url, self.title)
    }
}

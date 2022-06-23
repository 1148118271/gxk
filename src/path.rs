use std::env::Args;
use std::ffi::OsString;
use std::path::{Path, PathBuf};


static mut PATH: Option<OsString> = None;

pub fn get() -> &'static OsString {
    unsafe {
        if PATH.is_none() {
            let mut path = OsString::new();
            let mut args: Args = std::env::args();
            if let Some(v) = args.next() {
                let p = Path::new(v.as_str());
                if let Some(p) =  p.parent() {
                    let os_path = p.as_os_str();
                    if cfg!(windows) {
                        if os_path != "target\\debug" {
                            path = os_path.to_os_string()
                        }
                    } else {
                        if os_path != "target/debug" {
                            path = os_path.to_os_string()
                        }
                    }
                }
            }
            PATH = Some(path)
        }
        PATH.as_ref().unwrap()
    }
}

pub fn filling(p: &str) -> PathBuf {
    let path = get();
    if !path.is_empty() {
        Path::new(path).join(p).to_path_buf()
    } else {
        Path::new(p).to_path_buf()
    }
}


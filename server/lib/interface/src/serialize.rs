use graph::Path;
use base64;
use std::slice::from_raw_parts;


pub fn to_string(path : &Path) -> String {
    let slice = path.get_indices();
    base64::encode(unsafe {from_raw_parts(slice.as_ptr(), slice.len() * 8)})
}

pub fn to_path(path : &str) -> Path {
    Path::new(base64::decode(path).unwrap())
}

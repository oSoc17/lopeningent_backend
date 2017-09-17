use graph::Path;
use graph::NodeID;
use base64;
use std::slice::from_raw_parts;


pub fn to_string(path : &Path) -> String {
    let slice = path.get_indices();
    base64::encode(unsafe {from_raw_parts(slice.as_ptr() as *const u8, slice.len() * 8)})
}

pub fn to_path(path : &str) -> Path {
    let decoded = base64::decode(path).unwrap();
    let vec = unsafe {from_raw_parts(decoded.as_ptr() as *const NodeID, decoded.len() / 8)}.into_iter().cloned().collect();
    Path::new(vec)
}

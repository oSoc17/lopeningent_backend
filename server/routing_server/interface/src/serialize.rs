//! Basic functionality to create the route tag.

use graph::Path;
use graph::NodeID;
use base64;
use std::slice::from_raw_parts;
use std::error::Error;
use base64::URL_SAFE;

/// Path to tag.
pub fn to_string(path : &Path) -> String {
    let slice = path.get_indices();
    base64::encode_config(unsafe {from_raw_parts(slice.as_ptr() as *const u8, slice.len() * 8)}, URL_SAFE)
}

/// Tag to path.
pub fn to_path(path : &str) -> Result<Path, Box<Error>> {
    let decoded = base64::decode_config(path, URL_SAFE)?;
    let vec = unsafe {from_raw_parts(decoded.as_ptr() as *const NodeID, decoded.len() / 8)}.into_iter().cloned().collect();
    Ok(Path::new(vec))
}

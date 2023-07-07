use std::ffi::OsString;

pub fn convert_to_string(os_string: &OsString) -> String {
    match os_string.to_str() {
        None => panic!("The value should be of OsString type"),
        Some(str) => str.to_string(),
    }
}

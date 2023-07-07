use std::ffi::OsString;

pub fn convert_to_string(os_string: &OsString) -> String {
    os_string
        .to_str()
        .expect("the value should be of OsString type")
        .to_string()
}

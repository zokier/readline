extern crate "readline-sys" as ffi;
use std::io::IoResult;

use std::ffi::{CString, c_str_to_bytes};
use std::str;

pub fn readline(prompt: &str) -> IoResult<String> {
    use std::io::{IoError, IoErrorKind};
    unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way 
        let line_ptr = 
            ffi::readline(CString::from_slice(prompt.as_bytes()).as_ptr());

        if line_ptr.is_null() {
            return Err(IoError { 
                kind: IoErrorKind::EndOfFile,
                desc: "end of file",
                detail: None,
            });
        }

        let line = str::from_utf8(c_str_to_bytes(&(line_ptr as *const i8))).unwrap().to_string();

        Ok(line)
    }
}

pub fn add_history(line: &str) {
    let l = CString::from_slice(line.as_bytes()).as_ptr();
    unsafe { ffi::add_history(l as *const i8) };
}

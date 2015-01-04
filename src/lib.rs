extern crate "readline-sys" as ffi;
use std::io::IoResult;

use std::c_str::ToCStr;

pub fn readline(prompt: &str) -> IoResult<String> {
    use std::io::{IoError, IoErrorKind};
    unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way 
        let line_ptr = 
            ffi::readline(prompt.to_c_str_unchecked().as_ptr());

        if line_ptr.is_null() {
            return Err(IoError { 
                kind: IoErrorKind::EndOfFile,
                desc: "end of file",
                detail: None,
            });
        }


        let line = String::from_raw_buf(line_ptr as *const u8);

        Ok(line)
    }
}

pub fn add_history(line: &str) {
    unsafe { ffi::add_history(line.as_ptr() as *const i8) };
}

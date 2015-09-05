#![feature(alloc,libc,cstr_memory,cstr_to_str)]
extern crate readline_sys as readline_ffi;
extern crate libc;
extern crate alloc;

use std::ffi::{CStr,CString};
use libc::c_char;

pub enum ReadlineError {
    EndOfFile,
    InvalidUtf8(std::str::Utf8Error)
}

impl std::convert::From<std::str::Utf8Error> for ReadlineError {
    fn from(err: std::str::Utf8Error) -> ReadlineError {
        ReadlineError::InvalidUtf8(err)
    }
}

pub fn readline(prompt: &str) -> Result<String, ReadlineError> {
    unsafe {
        let line_ptr = readline_ffi::readline(
            prompt.as_ptr() as *const c_char);

        if line_ptr.is_null() {
            return Err(ReadlineError::EndOfFile);
        }

        return Ok(try!(CStr::from_ptr(line_ptr).to_str()).to_owned());
    }
}

pub fn add_history(line: &str) {
    unsafe {
        //TODO fix memory leak
        readline_ffi::add_history(CString::new(line).unwrap().into_raw());
    }
}

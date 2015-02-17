#![feature(alloc,libc,std_misc,core)]
extern crate "readline-sys" as readline_ffi;
extern crate libc;
extern crate alloc;

use std::ffi::c_str_to_bytes;
use libc::c_char;

/* copies the contents of given str to a new heap-allocated buffer
 * note: the returned buffer needs to be freed manually */
fn str_to_cstr(line: &str) -> *const c_char {
    let l = line.as_bytes();
    unsafe {
        //alignment, whats that?
        let b = alloc::heap::allocate(line.len()+1, 8);
        let s = std::slice::from_raw_parts_mut(b, line.len()+1);
        std::slice::bytes::copy_memory(s, l);
        s[line.len()] = 0;
        return b as *const c_char;
    }
}

pub enum ReadlineError {
    EndOfFile,
    InvalidUtf8(std::string::FromUtf8Error)
}

impl std::error::FromError<std::string::FromUtf8Error> for ReadlineError {
    fn from_error(err: std::string::FromUtf8Error) -> ReadlineError {
        ReadlineError::InvalidUtf8(err)
    }
}

pub fn readline(prompt: &str) -> Result<String, ReadlineError> {
    unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way 
        let line_ptr: *const c_char= 
            readline_ffi::readline(prompt.as_ptr() as *const c_char);

        if line_ptr.is_null() {
            return Err(ReadlineError::EndOfFile);
        }

        return Ok(try!(String::from_utf8(c_str_to_bytes(&line_ptr).to_vec())));
    }
}

pub fn add_history(line: &str) {
    unsafe {
        readline_ffi::add_history(str_to_cstr(line));
    }
}

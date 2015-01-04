extern crate "readline-sys" as ffi;
use std::io::IoResult;

use std::c_str::ToCStr;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn readline(prompt: &str) -> IoResult<String> {
    use std::io::{IoError, IoErrorKind};
    return unsafe {
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

        ffi::add_history(line_ptr as *const i8);

        let line = String::from_raw_buf(line_ptr as *const u8);

        Ok(line)
    }
}


#[cfg(target_os = "windows")]
pub fn readline(prompt: &str) -> IoResult<String> {
    use std::io::stdio::stdin;
    print!("{}", prompt);
    let line = stdin().read_line();
    match line {
        Ok(mut s) => {
            s.pop(); // takes the last \n off of the returned string
            Ok(s)
        },
        Err(e) => {
            Err(e)
        },
    }
}

extern crate "readline-sys" as ffi;

use std::c_str::ToCStr;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn readline(prompt: &str) -> Option<String> {
    return unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way 
        let line_ptr = 
            ffi::readline(prompt.to_c_str_unchecked().as_ptr());

        if line_ptr.is_null() {
            return None;
        }

        ffi::add_history(line_ptr as *const i8);

        let line = String::from_raw_buf(line_ptr as *const u8);

        Some(line)
    }
}


#[cfg(target_os = "windows")]
pub fn readline(prompt: &str) -> Option<String> {
    use std::io::stdio::stdin;
    use std::io::IoErrorKind;
    print!("{}", prompt);
    let line = stdin().read_line();
    match line {
        Ok(mut s) => {
            s.pop(); // takes the last \n off of the returned string
            Some(s)
        },
        Err(e) => {
            if e.kind == IoErrorKind::EndOfFile {
                return None;
            } else {
                panic!("IO Error! {}", e.desc);
            }
        },
    }
}

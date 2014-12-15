#[cfg(any(target_os = "linux", target_os = "macos"))]
mod readline_c {
    extern crate libc;
    use self::libc::c_char;
    extern {
        pub fn readline(prompt: *const c_char) -> *mut c_char;
        pub fn add_history(line: *const c_char);
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn readline(prompt: &str) -> Option<String> {
    return unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way 
        let line_ptr = 
            readline_c::readline(prompt.to_c_str_unchecked().as_ptr());

        if line_ptr.is_null() {
            return None;
        }

        readline_c::add_history(line_ptr as *const i8);

        let line = String::from_raw_buf(line_ptr as *const u8);

        Some(line)
    }
}


#[cfg(target_os = "windows")]
pub fn readline(prompt: &str) -> Option<&str> {
    use std::io::stdio::stdin;
    use std::io::IoErrorKind;
    unsafe { print!("{}", prompt); }
    let line = stdin().read_line();
    match line {
        Ok(mut s) => {
            s.pop(); // takes the last \n off of the returned string
            Some(s.to_c_str())
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

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern crate libc;
use self::libc::c_char;
extern "C" {
    pub fn readline(prompt: *const c_char) -> *mut c_char;
    pub fn add_history(line: *const c_char);
}

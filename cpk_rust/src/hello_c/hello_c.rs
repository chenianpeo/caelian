use std::ffi::{CStr, c_char};

pub fn hello(name: &str) {
    println!("hello {}, welcome to cpk", name);
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_ffi(name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name) };
    if let Ok(name_str) = name.to_str() {
        hello(name_str);
    }
}
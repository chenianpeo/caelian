use std::ffi::CStr;
use std::os::raw::c_char;

// write rust code
fn hello(name: &str) {
    println!("hello {name}, welcome to cpk");
}

// write C# code ffi api
#[unsafe(no_mangle)]
pub extern "C" fn ffi_hello(name: *const c_char) {
    if name.is_null() {
        return;
    }
    
    unsafe {
        let c_str = CStr::from_ptr(name);
        if let Ok(rust_str) = c_str.to_str() {
            hello(rust_str);
        }
    }
}
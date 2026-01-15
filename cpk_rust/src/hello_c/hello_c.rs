// write rust code
fn hello(name: &str) {
    print!("hello {name}, welcome to cpk");
}

// write C# code ffi api
#[unsafe(no_mangle)]
pub extern "C" fn ffi_hello(name: String) {
    hello(&name);
}
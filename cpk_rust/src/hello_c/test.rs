// write test code
#[cfg(test)]
mod test {
    // import test needy function
    use crate::hello_c::hello_c::ffi_hello;

    #[test]
    fn test_ffi_hello() {
        ffi_hello("joke".to_string());
    }
}
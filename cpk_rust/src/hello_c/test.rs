#[cfg(test)]
mod test {
    use crate::hello_c::hello_c::ffi_hello;

    #[test]
    fn test_ffi_hello() {
        ffi_hello("joke".to_string());
    }
}
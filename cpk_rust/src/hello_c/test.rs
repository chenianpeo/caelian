#[cfg(test)]
mod test {
    use crate::hello_c::hello_c::hello;

    #[test]
    fn test_hello_c() {
        hello("joke");
    }
}
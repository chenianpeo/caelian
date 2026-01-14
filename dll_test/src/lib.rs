// export c function that for C# using.
use std::os::raw::c_int;

#[unsafe(no_mangle)]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn add_float(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, -1), -2);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_float() {
        assert_eq!(add_float(2.5, 3.5), 6.0);
        assert_eq!(add_float(-1.5, 1.5), 0.0);
    }
}
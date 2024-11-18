//! # Rust example
//!

/// Returns the sum of 2 u64 values.
///
/// # Example
///
/// ```rust
/// use rs_example::add;
/// let result = add(1, 2);
/// println!("{}", result);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

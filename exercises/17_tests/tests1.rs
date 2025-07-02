// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    // Import everything from the outer module (the is_even function).
    use super::*;

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values.
        assert!(is_even(2)); // Test with an even number
        assert!(!is_even(3)); // Test with an odd number
    }
}

fn main() {
    // You can optionally experiment here.
}

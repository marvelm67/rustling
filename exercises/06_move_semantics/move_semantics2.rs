fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone(); // Make a copy of the input
    new_vec.push(88);

    new_vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66]; // `vec0` should be mutable for borrowing
        let vec1 = fill_vec(&mut vec0); // Borrow vec0 mutably

        // You can now access vec0 because it's not moved
        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

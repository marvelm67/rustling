fn main() {
    // Change the vector type to store `String` instead of `&str`
    let mut numbers: Vec<String> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.to_string()); // Convert `u8` to `String`
    let n2: i8 = -1;
    numbers.push(n2.to_string()); // Convert `i8` to `String`

    println!("{numbers:?}");
}

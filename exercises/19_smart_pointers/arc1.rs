use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // Use Arc to share ownership of numbers across threads
    let shared_numbers = Arc::new(numbers);
    let mut join_handles = Vec::new();

    // Spawning threads for each offset
    for offset in 0..8 {
        // Clone Arc to create a new reference, maintaining the reference count
        let child_numbers = shared_numbers.clone();
        
        // Spawn a new thread for each offset
        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        // Collect the join handles
        join_handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }

    // The reference count should now be correct for the test to pass
    println!("Expected reference count is 9, actual reference count: {}", Arc::strong_count(&shared_numbers));
}

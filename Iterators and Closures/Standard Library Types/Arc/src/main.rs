use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(
            thread::spawn(move || {
                let mut i = offset;
                let mut sum = 0;
                while i < child_numbers.len() {
                    sum += child_numbers[i];
                    i += 8;
                }
                println!("Sum of offset {} is {}", offset, sum);
            }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

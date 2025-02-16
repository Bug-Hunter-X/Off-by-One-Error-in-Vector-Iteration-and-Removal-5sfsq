fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Use iter_mut().enumerate() for safe index-based manipulation
    numbers.retain(|&x| x != 3);

    for number in &numbers {
        println!("Number: {}", number);
    }
}

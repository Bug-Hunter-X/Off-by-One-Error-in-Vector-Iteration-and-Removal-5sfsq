fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            break;
        }

        let number = numbers[index];
        println!("Number: {}", number);

        if number == 3 {
            numbers.remove(index);
        } else {
            index += 1;
        }
    }
}
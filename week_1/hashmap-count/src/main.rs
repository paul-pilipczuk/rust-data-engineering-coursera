use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, usize)> {
    // Create a HashMap to count the frequency of each number
    let mut counts = HashMap::new();

    // if you wanted to keep the same order of the tuples, you would need
    // another order collection (vector used)

    // Iterate through the numbers and count their occurrences
    for number in numbers {
        let counts = counts.entry(number).or_insert(0);
        *counts += 1;
    }
    // Convert the HashMap into a Vec of tuples (number, count)
    let mut result = Vec::new();

    for (num, count) in counts {
        result.push((num, count));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 2, 5];
    let result = logic(numbers);

    println!("Frequency of each number is: {:?}", result);
}

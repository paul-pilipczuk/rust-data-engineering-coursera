use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    // Vec Deque to hold fruits, allows for addition and removal from both ends
    let mut fruits: VecDeque<&str> = VecDeque::new();
    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Cherry");

    // Shuffle the fruits
    let mut rng = thread_rng();
    let mut fruits_vec: Vec<_> = fruits.into_iter().collect(); 
    fruits_vec.shuffle(&mut rng);

    // Convert back to VecDeque after shuffling
    let mut fruits: VecDeque<_> = fruits_vec.into_iter().collect(); // Convert back to VecDeque

    // Add more fruits to the salad from both ends
    fruits.push_front("Mango");
    fruits.push_back("Orange");
    fruits.push_back("Grapes");

    println!("Fruit Salad: {:?}", fruits);
}

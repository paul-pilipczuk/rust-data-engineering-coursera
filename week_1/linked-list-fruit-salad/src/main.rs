use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruits: LinkedList<&str> = LinkedList::new();
    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Elderberry");

    let mut rng = thread_rng();
    let mut fruits_vec: Vec<_> = fruits.into_iter().collect(); // Convert LinkedList to Vec
    fruits_vec.shuffle(&mut rng); // Shuffle the vector

    let mut fruits: LinkedList<_> = fruits_vec.into_iter().collect(); // Convert back to LinkedList

    fruits.push_front("Mango");
    fruits.push_back("Orange");
    fruits.push_back("Grapes");

    println!("Fruit Salad: {:?}", fruits);
}

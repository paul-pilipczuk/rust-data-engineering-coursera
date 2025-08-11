use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruits = vec![
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
        "Kiwi",
        "Lemon",
    ];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    //print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item);
        }
    }
}

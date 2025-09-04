use rand::seq::SliceRandom;
use rand::rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "apple".to_string(),
        "elderberry".to_string(),
        "pomegranate".to_string(),
        "kumquat".to_string(),
        "banana".to_string(),
        "grapefruit".to_string(),
        "kiwi".to_string(),
        "pear".to_string(),
    ];

    let mut rng = rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}

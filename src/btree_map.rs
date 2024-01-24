use std::collections::BTreeMap;

pub fn main() {
    // Creating a new BTreeMap
    let mut map = BTreeMap::new();

    // Inserting key-value pairs
    map.insert(3, "a");
    map.insert(1, "b");
    map.insert(2, "c");

    // Iterating over key-value pairs
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Searching for a key
    if let Some(value) = map.get(&2) {
        println!("Found value for key 2: {}", value);
    } else {
        println!("Key not found");
    }

    // Removing a key-value pair
    let mut removed = map.remove(&3);
    println!("Removed value: {:?}", removed.unwrap());

    println!("The remaining map looks like this: {:?}", map);
}

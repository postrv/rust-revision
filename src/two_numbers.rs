
use std::collections::HashMap;


pub fn main() {
    let nums = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let mut map = HashMap::new();
    let mut index: i32 = 0;
    for i in nums {
        if i < target {
            map.insert(i, index);
        }
        index += 1;
    }

    for (key, value) in &map {
        let mut pair_finder: i32 = target - key;
        if map.contains_key(&pair_finder) {
            println!("{}{:?}", value, map.get(&pair_finder).unwrap())
        }
    }
}


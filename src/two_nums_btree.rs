use std::collections::BTreeMap;

pub fn main() {
    let nums = vec![2, 7, 11, 15];
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    let target: i32 = 9;
    for (i, index) in nums.iter().enumerate() {
        if i < target as usize {
            map.insert(i as i32, *index);
        }
    }
    for (key, value) in &map {
        let mut pair_finder = key - target;
        if map.get(&pair_finder).is_some() {
            vec![value, map.get(&pair_finder).unwrap()];
        }
    }
}

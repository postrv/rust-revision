/*
Problem:
- Given a list of integers, use a vector and return the median(when sorted, the value
 in the middle position) and mode
  (the value that occurs most often; a hash map will be helpful here) of the list.
 */

use std::collections::HashMap;

pub fn main() {
    let mut vector_of_integers = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 3, 4, 5, 6, 7, 4, 5, 6, 7, 8, 9, 3, 4, 1,
    ]; //using vec! macro to create a vector
    let mut map = HashMap::new();
    for i in &vector_of_integers {
        let count = map.entry(i).or_insert(0);
        *count += 1
    }
    println!("{:?}", map); //prints the hashmap

    // Finding the mode
    let mut mode = 0;
    for count in map.values() {
        if *count > mode {
            mode = *count;
        }
    }
    println!("The mode is {}", mode);

    // Finding the least common value
    let mut min = 1;
    for count in map.values() {
        if *count != 0 && *count < min {
            min = *count;
        }
    }
    let mut min_count = map.get(&min).unwrap_or(&0); // get the value of the key min
    println!(
        "The least common value is {0}, which appears {1} times",
        min, min_count
    );

    // Finding the median
    let mut median = 0;
    let mut sorted_vector = &mut vector_of_integers.clone();
    sorted_vector.sort();
    let length = sorted_vector.len();
    if length % 2 == 0 {
        //if the length of the vector is even
        median = (sorted_vector[length / 2] + sorted_vector[length / 2 - 1]) / 2;
    } else {
        //if the length of the vector is odd
        median = sorted_vector[length / 2];
    }
    let mut median_count = 0;
    for i in sorted_vector {
        // Count the number of times the median appears from the sorted vector rather than from the hashmap
        if *i == median {
            median_count += 1;
        }
    }
    println!("The median is {0}, which is the value in the middle position when sorted and appears {1} times", median, median_count);
}

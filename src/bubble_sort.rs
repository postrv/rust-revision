// Implements bubble sort

fn bubble_sort(arr: &mut [i32]) { // takes array as mutable reference to array of i32 integers as argument
    let len = arr.len(); // finds the length of the input array
    for _ in 0..len { // the properties from outer loop are not used, hence the underscore
        for j in 0..len - 1 { // go through the array and stop before the very end
            // since we'll run out of pairs to run comparisons against
            if arr[j] > arr[j + 1] { // if element of array with the current index is
                // greater than the one to its right
                arr.swap(j, j + 1); //swap the element
            }
        }
    }
}

pub fn main() {
    let mut numbers = [5, 3, 8, 4, 2]; // mutable input array of i32 integers
    bubble_sort(&mut numbers); // function called
    println!("{:?}", numbers); // [2, 3, 4, 5, 8] // debug printed output - sorted array
}

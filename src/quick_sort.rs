fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot_index = partition(arr); // calls the below partition function which breaks
    // up the array into smaller sub arrays
    quick_sort(&mut arr[0..pivot_index]); // recursive call to quick sort to further divide
    // and order start of array
    quick_sort(&mut arr[pivot_index + 1..len]); // recursive call to quick sort to
    // further divide tail of array
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len(); // get the length of the array
    let pivot = arr[len / 2]; // set pivot to the middle of the array
    let mut i = 0; // pointer marking the index
    let mut j = len - 1; // pointer marking the end of the array but leaving out the last element
    loop { // infinite loop initialised
        while arr[i] < pivot { i += 1; } // increment the pointer at the start of the array
        // until the middle
        while arr[j] > pivot { j -= 1; } // decrement the pointer at the end of the array
        // until the middle
        if i >= j { return j; } // if earlier element is larger than later element return j
        // so that can be the start of the next recursive call
        arr.swap(i, j); // otherwise, swap the elements
        i += 1; // and increment the earlier pointer
        j -= 1; // and decrement the later pointer
    }
}

fn main() {
    let mut numbers = [3, 7, 8, 5, 2, 1, 9, 5, 4];
    quick_sort(&mut numbers);
    println!("{:?}", numbers); // [1, 2, 3, 4, 5, 5, 7, 8, 9]
}

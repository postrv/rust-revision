fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec(); // Create a copy of the left half
    let mut right = arr[mid..].to_vec(); // Create a copy of the right half

    merge_sort(&mut left); // Recursively sort the left half
    merge_sort(&mut right); // Recursively sort the right half

    // Merge the sorted halves back into arr
    merge(&left, &right, arr);
}

fn merge(left: &[i32], right: &[i32], arr: &mut [i32]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut arr_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            arr[arr_idx] = left[left_idx];
            left_idx += 1;
        } else {
            arr[arr_idx] = right[right_idx];
            right_idx += 1;
        }
        arr_idx += 1;
    }

    // Copy any remaining elements from left and right back to arr
    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx];
        left_idx += 1;
        arr_idx += 1;
    }

    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx];
        right_idx += 1;
        arr_idx += 1;
    }
}

pub fn main() {
    let mut numbers = [38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut numbers);
    println!("{:?}", numbers); // [3, 9, 10, 27, 38, 43, 82]
}

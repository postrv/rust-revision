use std::collections::BinaryHeap;

pub fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(1);
    heap.push(2);
    heap.push(3);
    println!("{:?}", heap);
    let ans = heap.into_sorted_vec();
    println!("{:?}", ans)
}


use std::collections::{LinkedList};



pub fn main() {
    let l1 = LinkedList::from([2, 4, 3]);
    let l2 = LinkedList::from([5,6,4]);
    // Next create iterators for each of the input lists as there isn't a built-in one

    let iter_1 = l1.iter();
    let iter_2 = l2.iter();

    let mut ans = LinkedList::new();

    loop {

    }
    println!("{:?}", ans)
}


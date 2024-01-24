# Linked Lists

Linked Lists offer an alternative to arrays. They allow for dynamic memory allocation, and efficient insertion and removal of elements at any position (not just the front or back like stacks/queues/dequeues).

## Types of Linked Lists

There are three forms of Linked List:

1. Singly linked lists: Each node contains data and a pointer to the next node in the sequence 
2. Doubly linked lists: Each node contains data and pointers to the previous and next node in the sequence 
3. Circular linked lists: Last node points back to the first node

## Rust implementation of Linked Lists

A singly linked list can be implemented as follows:

``` Rust
#[derive(Debug)] // only needed if printing the debug string  
enum List<T>{  
Entry,  
Node(T, Box<List<T>>),  
}


// To print the list, you'll need to derive Debug for the enum  
pub fn main () {  
use List::{Entry, Node};

    let list = Node(1,Box::new(Node(2,Box::new(Node(3,Box::new(Entry))))));  
    println!("{:?}", list)  
}
```

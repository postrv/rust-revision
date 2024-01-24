# Stacks, Queues, and Dequeues

In general, it's worth noting that in systems level programming languages, we are often concerned with both the
Space and Time complexity of our programs. We have to be much more cognisant of the memory requirements of our programs 
than we might need to be in GC languages.

This means memory needs to be allocated and deallocated efficiently.

## Stacks
Stacks are LIFO meaning the last element to be added is the first to be removed.

In Rust, they're implemented on a Vec as follows:

```Rust
let mut stack = Vec::new(); // Initialise a stack
stack.push(1); // Push an element onto the stack
stack.pop(); // Remove an element from the stack
```

## Queues
Queues are FIFO, meaning the first element to be added is the first to be removed.

They can be implemented in Rust using VecDeque from the standard library as follows:

```Rust
use std::collections::VecDeque;

let mut queue = VecDeque::new(); // Initialise the queue
queue.push_back(1); // Push a new element onto the back of the queue
queue.pop_front(); // Remove an element from the front of the queue
```

## Dequeue - VecDeque
VecDeque is a double-ended queue implemented with a growable ring buffer.

Since it's double-ended, elements can be added or removed from either end:

```Rust 
let mut deque = VecDeque::new();
deque.push_back(1); // Add to the back
deque.push_front(0); // Add to the front
deque.pop_back(); // Remove from the back
deque.pop_front(); // Remove from the front
```

#[derive(Debug)] // only needed if printing the debug string
enum List<T> {
    Entry,
    Node(T, Box<List<T>>),
}

// To print the list, you'll need to derive Debug for the enum
pub fn main() {
    use List::{Entry, Node};

    let list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Entry))))));
    println!("{:?}", list)
}

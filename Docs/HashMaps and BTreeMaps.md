# HashMaps and BTreeMaps

## HashMaps
The basic thesis of a HashMap is that it provides an efficient way to store and retrieve key-value pairs.

In order to work, there must be a 1:1 relationship/mapping between keys and values, such that one key returns exactly one value.

HashMaps in Rust
The Rust standard library provides the std::collections::HashMap module, which can be used for creating and interacting with HashMaps.

In the below example, we're storing the team name alongside their score. Since the team name is unique - there can only be one score for each team - this is the key, and the score, which could be the same for more than one team, is the value.
```Rust
use std::collections::HashMap;

let mut scores = HashMap::new(); // This initialises a new empty hashmap data structure

// To insert new key value pairs into the hashmap, the following can be used:

scores.insert(String::from("Blue"), 10); // Adding the Blue team, with a score of 10 points

scores.insert(String::from("Yellow"), 50); // Adding the Yellow team, with a score of 50 points
```

In order to retrieve values from the HashMap, we can use the get keyword:

```Rust
// Same setup as before to initialise the HashMap and add key value pairs to it
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Retrieve values from the above HashMap:

let team_name = String::from("Blue"); // Initialises a new variable called team_name of Type Strin

let score = scores.get(&team_name).copied().unwrap_or(0); // Borrows the team_name value, passes it into the `get` method of the HashMap module, finds the corresponding key value pair in the HashMap, copies it, then unwraps it or returns 0 if it can't be unwrapped (i.e. in the case where it doesn't return a valid value)
```

One can also loop through the HashMap to return the key value pairs in arbitrary order:

```Rust
// Same setup as before to initialise the HashMap and add key value pairs to it
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Loop through the HashMap and return the key value pairs:

for (key, value) in &scores { // borrows value from scores variable and loops through each key value pair within it
println!("{key}: {value}") // prints each key value pair
}
```


To update an existing value in a HashMap, we can simply insert a new value for the same key:

```Rust
scores.insert(String::from("Blue"), 10); // Setting original value
scores.insert(String::from("Blue"), 25)l // Setting a new value for the same key
The entry keyword in combination with or_insert() method can be used to add a key and value only if they don't already exist:

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10); // first value inserted as seen previously

scores.entry(String::from("Yellow")).or_insert(50); // Second value entered with `entry` keyword if not already present
scores.entry(String::from("Blue")).or_insert(50); // Will not insert the value 50 because the value 10 is already present for the key "Blue"

println!("{:?}", scores); // uses the ? operator in combinaton with the Debug trait on the underlying struct to print where the number of returned values is not known
```

If we want to update a value based on the old value (e.g. increment it) we can do the following:

```Rust
use std::collections::HashMap;

let text = "hello world wonderful world"; // initialises a new text string

let mut map = HashMap::new(); // initialises an empty HashMap

for word in text.split_whitespace() { // Splits the text string on whitespace and loops through each substring
let count = map.entry(word).or_insert(0); // Enters the key value pair {word: count} where
*count += 1; // dereferenced value `count` is incremented each time the same word comes up
}
println!("{:?}", map); // the resulting {word: count} key value pairs are printed
the output of this would look like this:

{"hello": 1, "world": 2, "wonderful": 1}
Exercises to program:

Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
/*
- Given a list of integers, use a vector and return the median(when sorted, the value  
  in the middle position) and mode  (the value that occurs most often; a hash map will be helpful here) of the list.
  */  
  use std::collections::HashMap;

pub(crate) fn main() {  
let mut vector_of_integers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 3, 4, 5, 6, 7, 4, 5, 6, 7, 8, 9, 3, 4, 1];//using vec! macro to create a vector  
let mut map = HashMap::new();  
for i in &vector_of_integers {  
let count = map.entry(i).or_insert(0);  
*count += 1  
}  
println!("{:?}", map); //prints the hashmap

    // Finding the mode    let mut mode = 0;  
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
    println!("The least common value is {0}, which appears {1} times", min, min_count);  
  
    // Finding the median  
    let mut median = 0;  
    let mut sorted_vector = &mut vector_of_integers.clone();  
    sorted_vector.sort();  
    let length = sorted_vector.len();  
    if length % 2 == 0 { //if the length of the vector is even  
        median = (sorted_vector[length / 2] + sorted_vector[length / 2 - 1]) / 2;  
    } else { //if the length of the vector is odd  
        median = sorted_vector[length / 2];  
    }  
    let mut median_count = 0;  
    for i in sorted_vector { // Get the number of times the median appears from the vector rather than from the hashmap  
        if *i == median {  
            median_count += 1;  
        }  
    }  
    println!("The median is {0}, which is the value in the middle position when sorted and appears {1} times", median, median_count);  
}
```

Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
```Rust
/*
- Convert strings to pig latin.  
  The first consonant of each word is moved to the end of the word and “ay” is added,  
  so “first” becomes “irst-fay.”  
  Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).  
  Keep in mind the details about UTF-8 encoding!  
  */  
  
use std::collections::HashMap;

pub fn main() {  
let first_string = "first";  
let first_letter = &first_string[0..1];  
let last_letters = &first_string[1..];  
let suffix = "ay";

    let mut map = HashMap::new();  // creates hashmap for the vowels
    map.insert("a", "hay");  
    map.insert("e", "hay");  
    map.insert("i", "hay");  
    map.insert("o", "hay");  
    map.insert("u", "hay");  
  
    let mut pig_latin_string = String::new();  
    if map.contains_key(first_letter) {  // if first letter is vowel
        pig_latin_string = first_string.to_owned() + "-" + map.get(first_letter).unwrap();  
    } else {  // if first letter is consonant
        pig_latin_string = last_letters.to_string() + "-" + first_letter + suffix;  
    }  
    println!("{}",pig_latin_string);  
}
```

Using a hash map and vectors, create a text interface to allow a user to add employee names to a 
department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
Then let the user retrieve a list of all people in a department or all people in the company by 
department, sorted alphabetically.


```Rust
/*
- Using a hash map and vectors, create a text interface to allow a user to add employee names  
  to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”  
  Then let the user retrieve a list of all people in a department or all people in the company  
  by department, sorted alphabetically.  
  */  
  pub fn main () {  
  use std::collections::HashMap;  
  use std::io;

  let mut company = HashMap::new();  
  let mut department = String::new();  
  let mut employee = String::new();  
  let mut input = String::new();  
  let mut department_list = Vec::new();  
  let mut employee_list = Vec::new();

  loop {  
  println!("Enter a command:");  
  input.clear();  
  io::stdin().read_line(&mut input).expect("Failed to read line");  
  let input = input.trim();  
  let input: Vec<&str> = input.split_whitespace().collect();  
  if input.len() == 4 && input[0] == "Add" && input[2] == "to" {  
  department = input[3].to_string();  
  employee = input[1].to_string();  
  company.entry(department.clone()).or_insert(Vec::new()).push(employee.clone());  
  department_list.push(department.clone());  
  employee_list.push(employee.clone());  
  } else if input.len() == 3 && input[0] == "List" && input[1] == "all" {  
  if input[2] == "departments" {  
  department_list.sort();  
  department_list.dedup();  
  println!("The departments are:");  
  for i in &department_list {  
  println!("{}", i);  
  }  
  } else if input[2] == "employees" {  
  employee_list.sort();  
  employee_list.dedup();  
  println!("The employees are:");  
  for i in &employee_list {  
  println!("{}", i);  
  }  
  } else {  
  println!("Invalid command");  
  }  
  } else if input.len() == 4 && input[0] == "List" && input[2] == "in" {  
  if input[3] == "department" {  
  department = input[3].to_string();  
  if company.contains_key(&department) {  
  println!("The employees in {} are:", department);  
  for i in company.get(&department).unwrap() {  
  println!("{}", i);  
  }  
  } else {  
  println!("{} is not a valid department", department);  
  }  
  } else {  
  println!("Invalid command");  
  }  
  } else {  
  println!("Invalid command");  
  }  
  }  
  }
```

## BTreeMaps

BTreeMaps are ordered maps based on B-Tree.

### Binary Search Trees
The following is an example implementation of Binary Search Tree:

```Rust 
#[derive(Debug)]  
pub struct TreeNode<T> {  
value: T, // The value of the node  
left: Option<Box<TreeNode<T>>>, // The left child of the node  
right: Option<Box<TreeNode<T>>>, // The right child of the node  
}

impl<T: Ord> TreeNode<T> {  
// Create a new TreeNode with the given value and no children  
fn new(value: T) -> Self {  
TreeNode { value, left: None, right: None }  
}

    // Insert a value into the tree  
    fn insert(&mut self, value: T) {  
        if value < self.value { // If the value is less than the value of the current node  
            match self.left { // Check if the left child exists  
                Some(ref mut left) => left.insert(value), // If it does, insert the value into the left child  
                None => self.left = Some(Box::new(TreeNode::new(value))), // If it doesn't, create a new node with the value and make it the left child  
            }  
        } else if value > self.value { // If the value is greater than the value of the current node  
            match self.right { // Check if the right child exists  
                Some(ref mut right) => right.insert(value), // If it does, insert the value into the right child  
                None => self.right = Some(Box::new(TreeNode::new(value))), // If it doesn't, create a new node with the value and make it the right child  
            }  
        }  
    }  
  
    // Search for a value in the tree  
    fn search(&self, value: T) -> bool { // Returns true if the value is found and false if it isn't  
        if value < self.value { // If the value is less than the value of the current node  
            match self.left { // Check if the left child exists  
                Some(ref left) => left.search(value), // If it does, search for the value in the left child  
                None => false, // If it doesn't, return false  
            }  
        } else if value > self.value { // If the value is greater than the value of the current node  
            match self.right { // Check if the right child exists  
                Some(ref right) => right.search(value), // If it does, search for the value in the right child  
                None => false, // If it doesn't, return false  
            }  
        } else { // If the value is equal to the value of the current node  
            true // Return true  
        }  
    }  
}

pub fn main() {  
let mut tree = TreeNode::new(5); // Create a new tree with the value 5  
tree.insert(3); // Insert the value 3 into the tree  
tree.insert(4); // Insert the value 4 into the tree  
tree.insert(2); // Insert the value 2 into the tree  
tree.insert(7); // Insert the value 7 into the tree  
tree.insert(6); // Insert the value 6 into the tree  
tree.insert(8); // Insert the value 8 into the tree  
println!("{:?}", tree); // Print the tree - not very useful for large trees because it's not very readable  
println!("{}", tree.search(4)); // Search for the value 4 in the tree and print the result  
println!("{}", tree.search(9)); // Search for the value 9 in the tree and print the result  
}
```

This:

- returns a printout of the tree
- finds 4 and returns true
- fails to find 9 and returns false


Prettier printout of the Tree

The following gives a tree-like representation in the terminal:

```Rust
// Prints a binary search tree in a pretty format in the terminal

use std::fmt::Display;  
use std::rc::Rc;  
use std::cell::RefCell;

type TreeLink<T> = Option<Rc<RefCell<PrettyTreeNode<T>>>>;

#[derive(Debug)]  
pub struct PrettyTreeNode<T> {  
value: T,  
left: TreeLink<T>,  
right: TreeLink<T>,  
}

impl<T: Ord + Display> PrettyTreeNode<T> {  
fn new(value: T) -> TreeLink<T> {  
Some(Rc::new(RefCell::new(PrettyTreeNode {  
value,  
left: None,  
right: None,  
})))  
}

    fn insert(root: &mut TreeLink<T>, value: T) {  
        if let Some(node) = root {  
            let mut node_borrowed = node.borrow_mut();  
            if value < node_borrowed.value {  
                PrettyTreeNode::insert(&mut node_borrowed.left, value);  
            } else if value > node_borrowed.value {  
                PrettyTreeNode::insert(&mut node_borrowed.right, value);  
            }  
        } else {  
            let new_node = PrettyTreeNode::new(value);  
            *root = new_node;  
        }  
    }  
  
    fn print_tree(node: &TreeLink<T>, prefix: String, is_left: bool) {  
        if let Some(node) = node {  
            let node = node.borrow();  
            let new_prefix = if is_left { "│   " } else { "    " };  
  
            PrettyTreeNode::print_tree(&node.right, format!("{}{}", prefix, new_prefix), false);  
            println!("{}{}── {}", prefix, if is_left { "└" } else { "┌" }, node.value);  
            PrettyTreeNode::print_tree(&node.left, format!("{}{}", prefix, new_prefix), true);  
        }  
    }  
}

pub fn main() {  
let mut root = PrettyTreeNode::new(10);

    PrettyTreeNode::insert(&mut root, 5);  
    PrettyTreeNode::insert(&mut root, 15);  
    PrettyTreeNode::insert(&mut root, 3);  
    PrettyTreeNode::insert(&mut root, 7);  
    PrettyTreeNode::insert(&mut root, 12);  
    PrettyTreeNode::insert(&mut root, 18);  
  
    PrettyTreeNode::print_tree(&root, "".to_string(), false);  
}
```

BTreeMap in Rust
The following is an example implementation of BTreeMap:

```Rust
use std::collections::BTreeMap;

pub fn main() {  
// Creating a new BTreeMap  
let mut map = BTreeMap::new();

    // Inserting key-value pairs  
    map.insert(3, "a");  
    map.insert(1, "b");  
    map.insert(2, "c");  
  
    // Iterating over key-value pairs  
    for (key, value) in &map {  
        println!("{}: {}", key, value);  
    }  
  
    // Searching for a key  
    if let Some(value) = map.get(&2) {  
        println!("Found value for key 2: {}", value);  
    } else {  
        println!("Key not found");  
    }  
  
    // Removing a key-value pair  
    let mut removed = map.remove(&3);  
    println!("Removed value: {:?}", removed.unwrap());  
  
    println!("The remaining map looks like this: {:?}", map);  

}
```
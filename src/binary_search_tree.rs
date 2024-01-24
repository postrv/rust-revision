#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,                        // The value of the node
    left: Option<Box<TreeNode<T>>>,  // The left child of the node
    right: Option<Box<TreeNode<T>>>, // The right child of the node
}

impl<T: Ord> TreeNode<T> {
    // Create a new TreeNode with the given value and no children
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a value into the tree
    fn insert(&mut self, value: T) {
        if value < self.value {
            // If the value is less than the value of the current node
            match self.left {
                // Check if the left child exists
                Some(ref mut left) => left.insert(value), // If it does, insert the value into the left child
                None => self.left = Some(Box::new(TreeNode::new(value))), // If it doesn't, create a new node with the value and make it the left child
            }
        } else if value > self.value {
            // If the value is greater than the value of the current node
            match self.right {
                // Check if the right child exists
                Some(ref mut right) => right.insert(value), // If it does, insert the value into the right child
                None => self.right = Some(Box::new(TreeNode::new(value))), // If it doesn't, create a new node with the value and make it the right child
            }
        }
    }

    // Search for a value in the tree
    fn search(&self, value: T) -> bool {
        // Returns true if the value is found and false if it isn't
        if value < self.value {
            // If the value is less than the value of the current node
            match self.left {
                // Check if the left child exists
                Some(ref left) => left.search(value), // If it does, search for the value in the left child
                None => false,                        // If it doesn't, return false
            }
        } else if value > self.value {
            // If the value is greater than the value of the current node
            match self.right {
                // Check if the right child exists
                Some(ref right) => right.search(value), // If it does, search for the value in the right child
                None => false,                          // If it doesn't, return false
            }
        } else {
            // If the value is equal to the value of the current node
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

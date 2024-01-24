// Prints a binary search tree in a pretty format in the terminal

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

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
            println!(
                "{}{}── {}",
                prefix,
                if is_left { "└" } else { "┌" },
                node.value
            );
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

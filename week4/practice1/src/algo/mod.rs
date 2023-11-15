use crate::node;
use std::collections::VecDeque;

pub fn depth_first_search<T>(node: &node::NodeRef<T>, value: T) -> Option<node::NodeRef<T>>
where
    T: PartialEq + Clone,
{
    if node.borrow().data == value {
        return Some(node.clone());
    }

    for child in &node.borrow().children {
        if let Some(found) = depth_first_search(child, value.clone()) {
            return Some(found);
        }
    }

    None
}

pub fn breadth_first_search<T>(node: &node::NodeRef<T>, value: T) -> Option<node::NodeRef<T>>
where
    T: PartialEq + Clone,
{
    let mut queue = VecDeque::new();
    queue.push_back(node.clone());

    while let Some(node) = queue.pop_front() {
        for child in &node.borrow().children {
            if child.borrow().data == value {
                return Some(child.clone());
            }
            queue.push_back(child.clone());
        }
    }
    None
}

pub fn traverse_tree<T, F>(node: &node::NodeRef<T>, callback: F)
where
    T: Clone,
    F: Fn(node::NodeRef<T>) -> () + Copy,
{
    callback(node.clone());
    for child in &node.borrow().children {
        traverse_tree(child, callback);
    }
}

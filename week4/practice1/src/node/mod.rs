use std::cell::RefCell;
use std::rc::{Rc, Weak};


pub type NodeRef<T> = Rc<RefCell<Node<T>>>;
pub type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;


#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub parent: Option<WeakNodeRef<T>>,
    pub children: Vec<NodeRef<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            parent: None,
            children: Vec::new()
        }
    }
}


#[allow(dead_code, unused_variables)]
pub fn new<T>(data: T) -> NodeRef<T> {
    Rc::new(RefCell::new(Node::new(data)))
}

#[allow(dead_code, unused_variables)]
pub fn append_child<T>(node: &NodeRef<T>, child: &NodeRef<T>) {
    node.borrow_mut().children.push(child.clone());
    child.borrow_mut().parent = Some(Rc::downgrade(node));
}

#[allow(dead_code, unused_variables)]
pub fn get_child<T>(node: &NodeRef<T>, index: usize) -> Option<NodeRef<T>> {
    node.borrow().children.get(index).cloned()
}

#[allow(dead_code, unused_variables)]
pub fn get_parent<T>(node: &NodeRef<T>) -> Option<NodeRef<T>> {
    node.borrow().parent.clone().and_then(|parent| parent.upgrade())
}

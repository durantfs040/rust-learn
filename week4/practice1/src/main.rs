mod node;
mod algo;

fn main() {
    let root = node::new(1);
    let child1 = node::new(2);
    let child2 = node::new(3);
    let child3 = node::new(4);
    let child4 = node::new(5);

    node::append_child(&root, &child1);
    node::append_child(&root, &child2);
    node::append_child(&child2, &child3);
    node::append_child(&child2, &child4);

    algo::traverse_tree(&root, |node| {
        println!("{}", node.borrow().data);
    });

    println!("{:?}", algo::depth_first_search(&root, 4));
    println!("{:?}", algo::breadth_first_search(&root, 6));
}

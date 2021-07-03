use algorithms::sorting::quicksort;
use algorithms::tree::BTNode;

fn main() {
    let unsorted = [5, 8, 1, 3, 2, 8];
    println!("Before: {:?}", unsorted);
    println!("After : {:?}", quicksort(&unsorted));

    let mut root = BTNode::new(5);
    root.insert_node(10);
    root.insert_node(2);
    root.insert_node(4);
    println!("{:#?}", root);
}

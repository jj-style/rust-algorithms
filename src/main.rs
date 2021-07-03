use algorithms::sorting::quicksort;
use algorithms::tree::Node;

fn main() {
    let unsorted = [5, 8, 1, 3, 2, 8];
    println!("Before: {:?}", unsorted);
    println!("After : {:?}", quicksort(&unsorted));

    let mut root = Node::with_data(5);
    root.insert_data(10);
    root.insert_data(2);
    root.insert_data(4);
    println!("{:#?}", root);
}

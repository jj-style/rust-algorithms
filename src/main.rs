mod sorting;
use sorting::bubble_sort;
use sorting::merge_sort;

fn main() {
    let mut arr: [isize; 5] = [5, 4, 3, 2, 1];

    println!("before: {:?}", arr);
    bubble_sort(&mut arr);
    merge_sort(&mut arr);
    println!("after: {:?}", arr);
}

use algorithms::sorting::quicksort;

fn main() {
    let unsorted = vec![5,8,1,3,2,8];
    println!("Before: {:?}", unsorted);
    println!("After : {:?}", quicksort(unsorted));
}

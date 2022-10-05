// using generic datatype T with Ord trait
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    // sorting numbers
    let mut numbers = [4, 65, 2, -31, 0, 2, 83, 1]; // change if required
    println!("Before: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);

    // sorting strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    insertion_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}

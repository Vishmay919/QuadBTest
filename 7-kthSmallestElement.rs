fn kth_smallest(arr: Vec<i32>, k: usize) -> Option<i32> {
    if arr.is_empty() || k == 0 || k > arr.len() {
        return None;
    }

    // Cloneing and sort the array
    let mut sorted_arr = arr.clone();
    sorted_arr.sort_unstable();

    // Returning the kth smallest element
    sorted_arr.get(k - 1).cloned()
}

fn main() {
    let numbers = vec![7, 10, 4, 3, 20, 15];
    let k = 3;
    match kth_smallest(numbers, k) {
        Some(val) => println!("The {}th smallest element is {}", k, val),
        None => println!("Invalid input"),
    }
}

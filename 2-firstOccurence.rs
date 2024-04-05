fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    // Using binary search to find the first occurrence
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // Checking if the target is found and returning its index
    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}

fn main() {
    let sorted_array = [1, 2, 4, 4, 5, 6, 7];
    let number_to_find = 4;
    match find_first_occurrence(&sorted_array, number_to_find) {
        Some(index) => println!("The first occurrence of {} is at index {}", number_to_find, index),
        None => println!("Number {} not found in the array", number_to_find),
    }
}

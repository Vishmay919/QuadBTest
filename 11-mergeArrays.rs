fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    // Creating iterators for both arrays
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();

    // Merging arrays in sorted order
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    // Appending the remaining elements
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

fn main() {
    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    println!("Merged array: {:?}", merge_sorted_arrays(&arr1, &arr2));
}

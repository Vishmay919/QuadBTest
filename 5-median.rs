fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        return f64::NAN; // Returning NaN if the array is empty
    }

    if len % 2 == 1 {
        // If the length is odd, returning the middle element
        arr[len / 2] as f64
    } else {
        // If the length is even, returning the average of the two middle elements
        (arr[len / 2 - 1] as f64 + arr[len / 2] as f64) / 2.0
    }
}

fn main() {
    let sorted_array = [1, 3, 3, 6, 7, 8, 9];
    println!("The median is {}", find_median(&sorted_array));
}

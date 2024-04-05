fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = std::i32::MIN;

    // Iterating through the array to find the maximum subarray sum
    for &x in arr {
        max_ending_here = max_ending_here + x;
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }

    max_so_far
}

fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum is {}", max_subarray_sum(&nums));
}

fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    // Sorting the vector of strings
    let mut sorted_strs = strs.clone();
    sorted_strs.sort_unstable();

    // Getting the first and the last string after sorting
    let first = sorted_strs.first().unwrap();
    let last = sorted_strs.last().unwrap();

    // Comparing characters of the first and the last string
    first.chars()
         .zip(last.chars())
         .take_while(|(a, b)| a == b)
         .map(|(a, _)| a)
         .collect()
}

fn main() {
    let strings = vec!["flower", "flow", "flight"];
    println!("The longest common prefix is '{}'", longest_common_prefix(strings));
}

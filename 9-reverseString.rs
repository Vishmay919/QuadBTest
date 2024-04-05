fn reverse_string(s: &str) -> String {
    // Collecting the characters of the string into a vector and reversing it
    s.chars().rev().collect()
}

fn main() {
    let original = "hello";
    println!("Original: {}", original);
    println!("Reversed: {}", reverse_string(original));
}

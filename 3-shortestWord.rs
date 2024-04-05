fn find_shortest_word(s: &str) -> Option<&str> {
    // Splitting the string into words and iterating to find the shortest one
    s.split_whitespace()
     .min_by_key(|word| word.len())
}

fn main() {
    let sentence = "Find the shortest word in this sentence";
    match find_shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is '{}'", shortest),
        None => println!("No words found"),
    }
}

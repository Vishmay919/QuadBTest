fn is_palindrome(input: &str) -> bool {
    // Cleaning the string: converting to lowercase and retaining alphanumeric characters only
    let clean_input: String = input.to_lowercase()
                                   .chars()
                                   .filter(|c| c.is_alphanumeric())
                                   .collect();

    // Comparing the cleaned string with its reverse
    clean_input == clean_input.chars().rev().collect::<String>()
}

fn main() {
    let test_string = "Madam, in Eden, I'm Adam";
    println!("Is '{}' a palindrome? {}", test_string, is_palindrome(test_string));
}

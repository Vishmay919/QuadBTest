fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false; // 0 and 1 are not prime numbers
    }
    if number <= 3 {
        return true; // 2 and 3 are prime numbers
    }

    // If the number is divisible by 2 or 3, it's not prime
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    // Checking for prime by testing divisibility up to the square root of the number
    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true 
}

fn main() {
    let num = 29;
    println!("Is {} prime? {}", num, is_prime(num));
}

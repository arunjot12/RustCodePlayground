
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false; // Not prime
    }
    if n <= 3 {
        return true; // 2 and 3 are prime
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false; // Divisible by 2 or 3, not prime
    }
    let mut i = 5;
    while i * i <= n { // Check divisors from 5 upwards
        if n % i == 0 || n % (i + 2) == 0 { // Check divisibility
            return false; // Found a divisor, not prime
        }
        i += 6; // Increment `i` by 6
    }
    true // No divisors found, prime
}

fn prime() {
    let number = 22; // Change this number to test other numbers
    if is_prime(number) {
        println!("{} is prime", number);
    } else {
        println!("{} is not prime", number);
    }
}
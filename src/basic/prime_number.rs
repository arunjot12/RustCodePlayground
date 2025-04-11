/// Check the number is prime or not
/// 
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

// pub fn prime() {
//     let number = 22; // Change this number to test other numbers
//     if is_prime(number) {
//         println!("{} is prime", number);
//     } else {
//         println!("{} is not prime", number);
//     }
// }

pub fn prime(){
    let mut start_number = String::new();
     std::io::stdin() .read_line(&mut start_number).expect("Failed to read line");
     let start_number: u32 =  start_number.trim().parse().expect("REASON");

     let mut last_number = String::new();
     std::io::stdin() .read_line(&mut last_number).expect("Failed to read line");
     let last_number: u32 =  last_number.trim().parse().expect("REASON");

     for i in start_number..last_number{
        let check_number = is_prime(i.try_into().unwrap());
        if check_number == true{
            println!("{}",i);
        }
     }
}

// Another approch
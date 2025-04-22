// Palindrome program with using in built functions
pub fn palindrome() {
    let number = 122;
    let rev: u32 = number
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .expect("not a number");
    if rev == number {
        println!("is a palindrome");
    } else {
        println!("not a palindrome")
    }
}

// Added the program without using inbuilt function
pub fn palindrome_number() {
    let mut num = 121;
    let original_number = num;
    let mut reversed = 0;
    while num > 0 {
        let digit = num % 10;
        reversed = reversed * 10 + digit;
        num /= 10;
    }
    if original_number == reversed {
        println!("Hence number is palindrome");
    } else {
        println!("Not")
    }
}

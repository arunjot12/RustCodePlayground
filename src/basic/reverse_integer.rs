// Program to reverse the integer
pub fn reverse_integer() {
    let mut number = -165;
    let original_number= number.to_string().chars().count();
    let char = original_number - 1;
    let mut reversed = 0;
    if number < 0 {
        for _ in 0..char  {
            let digit = number % 10;
            reversed = reversed * 10 + digit;
            number /= 10;
        }
    } else {
        while number > 0 {
            let digit = number % 10;
            reversed = reversed * 10 + digit;
            number /= 10;
        }
    }
    println!("The reverse integer is {}", reversed)
}

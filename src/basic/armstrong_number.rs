pub fn armstrong_number() {
    let original_number = 153;
    let mut number = original_number;
    let mut result = 0;

    while number != 0 {
        let remainder = number % 10;
        result += remainder * remainder * remainder;
        number /= 10;
    }

    if result == original_number {
        print!("is a armstrong number");
    } else {
        print!("not a armstrong number");
    }
}

// Arm strong through its length
//  fn main() {
//     for num in 1..=1000 {
//         if is_armstrong(num) {
//             println!("{}", num);
//         }
//     }
// }

// fn is_armstrong(num: u32) -> bool {
//     let num_str = num.to_string();
//     let num_digits = num_str.len() as u32;
//     let sum_of_powers: u32 = num_str
//         .chars()
//         .map(|c| c.to_digit(10).unwrap().pow(num_digits))
//         .sum();
//     sum_of_powers == num
// }

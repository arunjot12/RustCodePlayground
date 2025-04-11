use std::{io,sync::RwLock };
use lazy_static::lazy_static;


// Main function input


// #[cfg(feature = "swap_loop")]
// swap_loop();
// // Loop to repeatedly swap numbers based on user input
// loop {
//     swap_numbers_using_input();
//     println!("Do you want to swap the numbers again? (yes/no)");
//     let mut response = String::new();
//     io::stdin().read_line(&mut response).unwrap();
//     let response = response.trim().to_lowercase();

//     if response != "yes" {
//         break;
//     }
// }

#[allow(dead_code)]
/// A normal swap between the two variables
pub fn swap_numbers() {
    let mut number_one = 1;
    let mut number_second = 3;
    println!("number_one {},number_two {}", number_one, number_second);
    let swap_second = number_one;
    number_one = number_second;
    number_second = swap_second;
    println!("number_one {},number_two {}", number_one, number_second);
}

lazy_static! {
    static ref FIRST_NUMBER: RwLock<u32> = RwLock::new(0);
    static ref SECOND_NUMBER: RwLock<u32> = RwLock::new(0);
}

// Swap without using the third variable

pub fn swap(){
    let mut  first_number = 1;
    let mut second_number = 5;
    first_number = first_number + second_number;
    second_number = first_number - second_number;
    first_number = first_number - second_number;
    println!("{}",first_number);
}

#[cfg(feature = "swap_loop")]
/// Swap using the user input
pub fn swap_numbers_using_input() {
    println!("Please choose if you want to input or want to Show the existing numbers ");

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: String = command.to_lowercase().trim().parse().unwrap();

        match command.as_ref() {
            "show" => {
                show();
                break;
            }
            "input" => {
                input();
                break; // Exit the loop after input
            }
            _ => println!("invalid input"),
        }
    }

    fn show() {
        println!("First number: {:?}", *FIRST_NUMBER);
        println!("Second number: {:?}", *SECOND_NUMBER);
    }

    fn input() {
        println!("Please inter the first number");

        let mut input_one = String::new();
        io::stdin().read_line(&mut input_one).expect("Failed to read line");
        let first_number: u32 = input_one.trim().parse().expect("Please type a number!");

        println!("Please inter the second number");
        let mut second_one = String::new();
        io::stdin().read_line(&mut second_one).expect("Failed to read line");
        let second_number: u32 = second_one.trim().parse().expect("Please type a number in second integer!");

        *FIRST_NUMBER.write().unwrap() = second_number;
        *SECOND_NUMBER.write().unwrap() = first_number;

        println!("thank you for storing the numbers");
    }
}

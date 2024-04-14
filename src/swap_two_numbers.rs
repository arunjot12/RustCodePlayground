use std::{io, process::exit};
use lazy_static::lazy_static;

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

/// Swap using the user input
pub fn swap_numbers_using_input() {
    println!("Please choose if you want to input or want to Show the existing numbers ");

    lazy_static! {
        static ref FIRST_NUMBER: u32 = 0;
        static ref SECOND_NUMBER: u32 = 0;
    }
    

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
        println!("First number: {}", *FIRST_NUMBER);
        println!("Second number: {}", *SECOND_NUMBER);
    }

    fn input() {
        println!("Please inter the first number");

        let mut input_one = String::new();
        io::stdin().read_line(&mut input_one).expect("Failed to read line");
        let mut ref_first :u32 = *FIRST_NUMBER;
        ref_first = input_one.trim().parse().expect("Please type a number!");

        println!("Please inter the second number");
        let mut second_one = String::new();
        io::stdin().read_line(&mut second_one).expect("Failed to read line");
        let mut ref_second:u32 = *SECOND_NUMBER;
        ref_second = second_one
            .trim()
            .parse()
            .expect("Please type a number in second integer!");

        let swap_second_number = ref_first;
        ref_first = ref_second;
        ref_second = swap_second_number;

        println!("thank you for storing the numbers");
        
    }
}

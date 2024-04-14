pub mod swap_two_numbers;
use std::{io, process::exit};
use crate::swap_two_numbers::swap_numbers_using_input;
fn main() {
    swap_numbers_using_input();
    loop {
        swap_numbers_using_input();
        println!("Do you want to show the numbers again? (yes/no)");

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response: String = response.trim().to_lowercase();

        if response != "yes" {
            break;
        }
    }
}

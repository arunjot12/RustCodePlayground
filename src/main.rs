// Declare modules
pub mod basic;
pub mod patterns;
use crate::patterns::pyramid_numbers::pyramid_numbers;

fn main() { 
    pyramid_numbers()

    // Loop to repeatedly swap numbers based on user input
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
}

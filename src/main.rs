// Declare modules
pub mod basic;
pub mod patterns;
pub mod arrays;
use arrays::largest_array::largest_number_array;

// use basic::lcm::lcm;

fn main() { 
    // lcm()
    largest_number_array();

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

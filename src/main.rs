// Declare modules
pub mod basic;
pub mod patterns;
pub mod arrays;
use arrays::sum_array::two_sum;
// use arrays::zombie_array::create_zombie;

fn main() { 
    // lcm()
    // unsafe { create_zombie() };
    two_sum();
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

// Declare modules
pub mod rust_topics;
pub use rust_topics::threads::threads;
// use enums::rust_enum::rust_enum;
// use arrays::zombie_array::create_zombie;

fn main() { 
    // rust_enum();
    threads();
    
    // lcm()
    // unsafe { create_zombie() };
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

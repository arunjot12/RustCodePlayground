// Declare all modules
pub mod arrays;
pub mod async_programming;
pub mod basic;
#[allow(non_snake_case)]
pub mod Hashmaps;
pub mod linked_list;
pub mod memory_management;
pub mod patterns;
pub mod rust_libraries;
pub mod rust_topics;

use std::io::{self, Write};

macro_rules! run_menu {
    ($title:expr, $(($name:expr, $func:path)),* $(,)?) => {
        #[allow(unused_assignments)]
        loop {
            println!("\n--- {} ---", $title);
            let mut i = 1;
            $(
                println!("{}. {}", i, $name);
                i += 1;
            )*
            println!("0. Back");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim();

            if choice == "0" {
                break;
            }
            
            let mut matched = false;
            let mut current = 1;
            $(
                if choice == current.to_string() {
                    println!("\nRunning: {}\n", $name);
                    $func();
                    matched = true;
                }
                current += 1;
            )*


            if !matched {
                println!("Invalid choice. Please try again.");
            }
        }
    };
}

fn run_arrays() {
    run_menu!(
        "Arrays",
        ("Two D Array", arrays::two_d_array::two_d_array),
        ("Max Min Array", arrays::max_min_array::max_min_array),
        ("Reverse String", arrays::reverse_string::reverse_string),
        ("Reverse Array", arrays::reverse_array::reverse),
        ("Reverse Array 2", arrays::reverse_array::reverse_array),
        ("Largest Array", arrays::largest_array::largest_number_array),
        ("Removing Duplicasy", arrays::removing_duplicasy::removing_duplicasy),
        ("Three D Arrays", arrays::three_d_arrays::three_d_arrays),
        ("Largest String Array", arrays::largest_string_array::largest_array),
        ("Largest String Array 2", arrays::largest_string_array::largest_string_array),
        ("Sum Array", arrays::sum_array::sum_of_array),
        ("Search Array", arrays::search_array::search_array),
    );
}

fn run_basic() {
    run_menu!(
        "Basic Programs",
        ("Swap Numbers", basic::swap_two_numbers::swap_numbers),
        ("Swap Numbers Using Input", basic::swap_two_numbers::swap_numbers_using_input),
        ("Multiplication Table", basic::multiplication_table::multiplication_table),
        ("Armstrong Number", basic::armstrong_number::armstrong_number),
        ("Odd Number", basic::odd_number::odd_number),
        ("Calculator", basic::calculator::calculate),
        ("Reverse Integer", basic::reverse_integer::reverse_integer),
        ("Largest Number", basic::largest_number::largest_number),
        ("Prime Number", basic::prime_number::prime),
        ("Leap Year", basic::leap_year::leap_year),
        ("LCM", basic::lcm::lcm),
        ("Palindrome", basic::palindrome_number::palindrome),
        ("Palindrome Number", basic::palindrome_number::palindrome_number),
        ("Fibonacci", basic::fibonacci::fibonacci),
        ("Reverse Number", basic::reverse_number::reverse),
        ("Factorial", basic::factorial_number::factorial),
    );
}

fn run_hashmaps() {
    run_menu!(
        "Hashmaps",
        ("Hashmap", Hashmaps::hashmap::hashmap),
        ("Hashmap Entry", Hashmaps::hashmap::hashmap_entry),
        ("Dereference Array", Hashmaps::hashmap::deference_array),
    );
}

fn run_patterns() {
    run_menu!(
        "Patterns",
        ("Simple Pyramid", patterns::simple_pyramid::simple_pyramid),
        ("Reverse Pyramid", patterns::simple_pyramid::reverse_pyramid),
        ("Square Pyramid", patterns::square_pyramid::square_pyramid),
        ("Pyramid Numbers", patterns::pyramid_numbers::pyramid_numbers),
        ("Full Pyramid", patterns::full_pyramid::full_pyramid),
        ("Hollow Diamond", patterns::hollow_diamond_pyramid::hollow_diamond),
        ("Hollow Pyramid", patterns::hollow_pyramid::hollow),
    );
}

fn run_memory() {
    run_menu!(
        "Memory Management",
        ("Threads", memory_management::threads::thread),
        ("Memory Allocation Size", memory_management::memory_allocation::size),
    );
}

fn run_topics() {
    run_menu!(
        "Rust Topics",
        ("Enums", rust_topics::enums::rust_enum),
        ("Lifetimes", rust_topics::lifetimes::lifetime),
        ("References", rust_topics::references::reference),
        ("Structs", rust_topics::structs::reference),
        ("Generics", rust_topics::generics::use_of_generic),
        ("Option Unwrap", rust_topics::option::option_unwrap),
    );
}

fn run_linked_list() {
    run_menu!(
        "Linked List",
        ("New List", linked_list::new_list::new_list),
    );
}

fn run_async() {
    run_menu!(
        "Async Programming",
        ("Hello Async", async_programming::asyncs::hello),
    );
}

fn run_libraries() {
    run_menu!(
        "Rust Libraries",
        ("Cronjob", rust_libraries::cronjob::cron),
    );
}

fn main() {
    loop {
        println!("\n🚀 Welcome to the Rust World!");
        println!("Please choose a category to run:");
        println!("1. Arrays");
        println!("2. Async Programming");
        println!("3. Basic Programs");
        println!("4. Hashmaps");
        println!("5. Linked List");
        println!("6. Memory Management");
        println!("7. Patterns");
        println!("8. Rust Libraries");
        println!("9. Rust Topics");
        println!("0. Exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => run_arrays(),
            "2" => run_async(),
            "3" => run_basic(),
            "4" => run_hashmaps(),
            "5" => run_linked_list(),
            "6" => run_memory(),
            "7" => run_patterns(),
            "8" => run_libraries(),
            "9" => run_topics(),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

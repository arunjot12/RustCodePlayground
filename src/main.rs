// Declare modules
pub mod basic;
pub mod linked_list;
pub mod memory_management;
pub mod rust_topics;
pub use memory_management::*;
pub mod async_programming;
pub use crate::async_programming::*;

#[cfg(feature = "linked-list")]
use crate::linked_list::new_list;
#[allow(unused)]
use crate::threads::thread;
pub use rust_topics::*;

fn main() {
    println!("🚀 Welcome to the Rust World!");
    println!("🔧 Please enable the desired feature(s) to run specific program.");

    #[cfg(feature = "linked-list")]
    new_list();
}

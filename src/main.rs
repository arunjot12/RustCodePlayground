// Declare modules
pub mod basic;
pub mod rust_topics;
pub mod memory_management;
pub use memory_management::*;
#[allow(unused)]
use crate::threads::thread;
pub use rust_topics::*;

fn main() {
    println!("🚀 Welcome to the Rust World!");
    println!("🔧 Please enable the desired feature(s) to run specific program.");
    
    #[cfg(feature = "threads")]
    thread();
}

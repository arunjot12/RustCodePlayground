// Declare modules
pub mod basic;
pub mod rust_topics;
#[allow(unused)]
use crate::memory_allocation::size;
pub use rust_topics::*;

fn main() {
    println!("🚀 Welcome to the Rust World!");
    println!("🔧 Please enable the desired feature(s) to run specific program.");
    
    #[cfg(feature = "memory-allocation")]
    size();
}

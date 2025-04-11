// Declare modules
pub mod rust_topics;
pub mod basic;
pub use rust_topics::*;

fn main() { 

    #[cfg(feature = "rust_enum")]
    rust_enum();
}

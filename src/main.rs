// Declare modules
pub mod basic;
pub mod rust_topics;
#[allow(unused)]
use crate::option::option_unwrap;
pub use rust_topics::*;

fn main() {
    #[cfg(feature = "option")]
    option_unwrap();
}

// Declare modules
pub mod basic;
pub mod rust_topics;
use crate::generics::use_of_generic;
pub use rust_topics::*;

fn main() {
    #[cfg(feature = "generics")]
    use_of_generic();
}

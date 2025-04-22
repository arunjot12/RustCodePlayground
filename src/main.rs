// Declare modules
pub mod basic;
pub mod rust_topics;
use crate::references::reference;
pub use rust_topics::*;

fn main() {
    #[cfg(feature = "reference")]
    reference();
}

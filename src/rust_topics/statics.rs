// =====================================
//         RUST STATIC VARIABLES
// =====================================

/*
`static` variables in Rust are global constants with a fixed memory address.
They live for the entire duration of the program.
They must have a fixed type and require `unsafe` to mutate.
*/

// -------------------------------------
// Declaration of Static Variables
// -------------------------------------

static A: u32 = 3; // Unsigned 32-bit integer
static B: i32 = -1_000_000; // Signed 32-bit integer with visual separator
static C: f64 = 5.7e10; // 64-bit floating-point number in scientific notation
static D: u8 = 200; // Unsigned 8-bit integer

// -------------------------------------
// Usage Function
// -------------------------------------

pub fn show_statics() {
    println!("--- Statics Demo ---");
    println!("Static A: {}", A);
    println!("Static B: {}", B);
    println!("Static C: {}", C);
    println!("Static D: {}", D);
}

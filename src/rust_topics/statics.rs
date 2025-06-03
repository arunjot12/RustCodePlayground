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

static _A: u32 = 3; // Unsigned 32-bit integer
static _B: i32 = -1_000_000; // Signed 32-bit integer with visual separator
static _C: f64 = 5.7e10; // 64-bit floating-point number in scientific notation
static _D: u8 = 200; // Unsigned 8-bit integer

// -------------------------------------
// Usage Function
// -------------------------------------

#[allow(unused)]
fn show_static_variables() {
    println!("Static _A: {}", _A);
    println!("Static _B: {}", _B);
    println!("Static _C: {}", _C);
    println!("Static _D: {}", _D);
}

// =====================================
//           RUST REFERENCES
// =====================================

/*
References (& and &mut) allow you to borrow data without taking ownership.
This helps with memory safety and performance without needing to clone data.
*/

// -------------------------------------
// Example 1: Using Mutable Reference in a Function
// -------------------------------------

/// This function is enabled with the "reference" feature flag.
/// Demonstrates modifying elements of an array through a mutable reference.
#[cfg(feature = "reference")]
fn reference_array_example() {
    let mut arr = [1, -5, 12, -98];
    double_negative(&mut arr);
    println!("{:?}", arr); // Output: [1, -10, 12, -196]
}

/// Doubles each negative number in the array via a mutable reference.
fn _double_negative(a: &mut [i16; 4]) {
    for i in a {
        if *i < 0 {
            *i *= 2;
        }
    }
}

// -------------------------------------
// Example 2: Basic Mutable Reference
// -------------------------------------

/// Demonstrates mutable references and how they modify original values.
pub fn reference() {
    let mut x = 10;

    let y = &mut x; // Borrow x as mutable reference
    println!("Before modification: {}", y);

    *y += 5; // Modify x through the reference

    println!("After modification: {}", x); // x is now 15
}

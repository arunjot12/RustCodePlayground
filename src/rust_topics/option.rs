// =====================================
//          RUST OPTION EXAMPLES
// =====================================

/*
The `Option` enum in Rust is used to represent values that may or may not be present.
It has two variants:
  - Some(T): contains a value of type T
  - None: represents absence of a value
*/

// -------------------------------------
// Example 1: Using Option with pop()
// -------------------------------------

/// Demonstrates how `Option` is used when popping values from a vector.
/// Prints each value until the vector is empty, then prints "#".
fn _option() {
    let mut data = vec![1, 2, 3];

    for _ in 0..5 {
        let item = data.pop();
        match item {
            Some(value) => println!("{}", value),
            None => println!("#"), // Indicates no more elements to pop
        }
    }
}

// -------------------------------------
// Example 2: Option with unwrap and match
// -------------------------------------

/// Demonstrates safe access to an `Option` using match.
/// `unwrap()` is not used directly to avoid panics in case of `None`.
pub fn option_unwrap() {
    let data: Option<u32> = None;

    println!("{:?}", data); // Prints: None

    match data {
        Some(value) => println!("{}", value),
        None => {
            // Graceful handling of None without panic
        }
    }
}

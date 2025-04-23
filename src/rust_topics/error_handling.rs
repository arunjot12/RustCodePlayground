// =====================================
//        RUST ERROR HANDLING
// =====================================

/*
Rust uses the `Result` enum for recoverable errors.
It has two variants:
  - Ok(T): contains a successful result
  - Err(E): contains an error message or type
*/

// -------------------------------------
// Example: Division with Result
// -------------------------------------

/// Divides `numerator` by `denominator`.
/// Returns `Ok(result)` if valid, otherwise returns an `Err` with an error message.
fn divide(numerator: u32, denominator: u32) -> Result<u32, String> {
    if denominator == 0 {
        Err(format!("Number is zero"))
    } else {
        Ok(numerator / denominator)
    }
}

/// Uses the `divide` function and matches the result.
/// Prints the result or error message accordingly.
fn show_divide(num: u32, den: u32) {
    match divide(num, den) {
        Ok(value) => println!("{}/{} = {}", num, den, value),
        Err(msg)  => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}

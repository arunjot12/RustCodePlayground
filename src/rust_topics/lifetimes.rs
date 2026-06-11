/// Demonstrates basic usage of string slices and lifetime annotations.
pub fn lifetime() {
    let a = "arunjot"; // string literal, inherently has a 'static lifetime
    let b = "singh"; // string literal, inherently has a 'static lifetime
    let _c = "arunjot";
    let _amit = String::from("abhi"); // owned String on the heap

    // The result gets its lifetime from `a` and `b`, which both live long enough
    // for `result` to be printed.
    let result = longest_string(a, b);
    println!("Longest string: {}", result);
}

/// Returns the longest of two string slices.
///
/// The lifetime annotation `'a` tells the Rust compiler that the returned
/// reference will live at least as long as the shortest lifetime of the
/// input references `a` and `b`. This prevents dangling references.
pub fn longest_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

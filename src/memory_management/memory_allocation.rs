// =====================================
//         RUST MEMORY & SIZE
// =====================================

/*
This example explores:
- Size of primitive types
- Memory layout of `String`
- UTF-8 encoding in Rust
*/

#[allow(unused)]
use std::mem::size_of;

/// Prints size of various primitive types.
pub fn size() {
    let _a: u8 = 255;
    let _b: i8 = -128;

    println!("Size of u8  (_a): {} bytes", size_of::<u8>());
    println!("Size of i8  (_b): {} bytes", size_of::<i8>());
    println!("Size of usize: {} bytes", size_of::<usize>());
    println!("Size of isize: {} bytes", size_of::<isize>());
}

// -------------------------------------
// Memory Layout of String in Rust
// -------------------------------------

/*
Internally, a String is represented as:

struct String {
    ptr: *const u8,    // Pointer to the buffer (heap memory)
    len: usize,        // Length of the string (in bytes)
    capacity: usize,   // Total allocated capacity
}

On a 64-bit system:
- ptr: 8 bytes
- len: 8 bytes
- capacity: 8 bytes
=> Total: 24 bytes for the String struct itself (stack).
*/

// -------------------------------------
// UTF-8 Encoding Behavior in Strings
// -------------------------------------

#[allow(unused)]
fn string_memory_usage() {
    let ascii = String::from("hello");
    println!("ASCII String: {}", ascii);
    println!("Bytes used (ascii): {}", ascii.len()); // Output: 5

    let unicode = String::from("हेलो");
    println!("Unicode String: {}", unicode);
    println!("Bytes used (unicode): {}", unicode.len()); // Output: 15
}

/*
TL;DR:
- `.len()` gives the number of bytes used (not characters).
- ASCII: 1 byte per character.
- Unicode (e.g., Devanagari, emojis): 2–4 bytes per character.
- To get total memory: size_of::<String>() + s.capacity()
*/

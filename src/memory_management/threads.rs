// =====================================
//         RUST THREADING BASICS
// =====================================

/*
This example demonstrates how to create and manage threads in Rust
using `std::thread::spawn()` and `join()`.
*/

use std::thread;

/// Spawns a new thread and waits for it to complete.
pub fn thread() {
    // Spawn a new thread using a closure
    let handle = thread::spawn(|| {
        println!("Hello from another thread!");
    });

    // This message prints from the main thread
    println!("Hello from the main thread!");

    // Wait for the spawned thread to finish
    handle.join().unwrap(); // `.join()` returns a Result
}

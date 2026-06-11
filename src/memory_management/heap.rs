#[allow(dead_code)]
fn heap() {
    std::thread::spawn(|| {
        // This is a new thread.
        let _a = 10; // Stored on this thread's stack
        let _b = Box::new(20); // Stored on heap, shared memory space
    });
}

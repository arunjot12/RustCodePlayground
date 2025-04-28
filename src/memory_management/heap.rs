fn heap() {
    std::thread::spawn(|| {
        // This is a new thread.
        let a = 10; // Stored on this thread's stack
        let b = Box::new(20); // Stored on heap, shared memory space
    });
}

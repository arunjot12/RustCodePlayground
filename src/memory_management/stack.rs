//  Stack
// The stack is fast, contiguous memory.

// It stores:

// Local variables with known size at compile time

// Return addresses for function calls (call frames)

fn stack() {
    let x = 42; // stored on the stack
}
/// Stack allocation is very fast (just a pointer move), and deallocation happens automatically when a function returns (LIFO: last-in, first-out).
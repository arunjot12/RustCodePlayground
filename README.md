# Rust Code Playground

A collection of Rust programs demonstrating various concepts and features of the Rust programming language. This repository is ideal for learners and enthusiasts looking to explore Rust through hands-on examples.

## 📁 Folder Structure

The codebase is organized into folders, each focused on different aspects of Rust programming:

### 🔹 `basic/`
Contains beginner-friendly programs that cover:
- Input/output handling
- Basic control flow (if/else, loops)
- Variable declaration and data types

### 🔹 `rust_topics/`
Covers core Rust topics such as:
- Ownership and Borrowing
- Structs and Enums
- Traits and Implementations
- Pattern Matching
- Error handling (`Result`, `Option`)

### 🔹 `arrays/`
Includes examples related to:
- Array and Vector usage
- Iterating over collections
- Array manipulation techniques

Each folder includes multiple `.rs` files with self-contained Rust programs.

### 🔹 `memory_management/`
Covers memory handling and low-level behavior in Rust:
- **Stack vs Heap**: How Rust decides where to allocate memory and the implications of each.
- **Memory Allocation**: Using `Box`, `Rc`, and smart pointers to allocate memory on the heap and share ownership.
- **Lifetimes**: Ensuring memory safety at compile time through lifetime annotations.
- **Threads & Concurrency**:
  - Spawning and managing threads using `std::thread`
  - Sharing memory safely between threads using `Arc` and `Mutex`
  - Demonstrating Rust’s fearless concurrency model

---


## 🚀 Getting Started

Make sure you have [Rust installed](https://rustup.rs/).

### Clone the repository:
```bash
git clone https://github.com/arunjot12/Rust_programs.git
cd Rust_programs

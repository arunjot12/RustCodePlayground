// =====================================
//              RUST ENUMS
// =====================================

/*
Enums in Rust are used to define a type which can be one of several variants.
They are powerful and commonly used in pattern matching.
*/

// -------------------------------------
// Example 1: Basic Enum with a Single Variant
// -------------------------------------

#[cfg(feature = "rust_enum")]
pub fn rust_enum() {
    enum Office {
        Employee(String),
    }

    let employee_data = Office::Employee("Alice".to_string());

    match employee_data {
        Office::Employee(employee) => println!("Employee: {}", employee),
    }
}

// -------------------------------------
// Example 2: Enum with Multiple Variants
// -------------------------------------

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Moving north!"),
        Direction::South => println!("Heading south!"),
        Direction::East  => println!("Going east!"),
        Direction::West  => println!("Turning west!"),
    }
}

// -------------------------------------
// Example 3: Enum with Data in Variants
// -------------------------------------

enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received."),
        Message::Write(text) => println!("Message: {}", text),
        Message::Move { x, y } => println!("Moving to position ({}, {})", x, y),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

// -------------------------------------
// Example 4: Enum with Generic Result Type
// -------------------------------------

fn use_result_enum() {
    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(val) => println!("Got value: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}

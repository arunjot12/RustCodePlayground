// =====================================
//          RUST GENERICS EXAMPLES
// =====================================

/*
Generics allow for writing flexible and reusable code. 
They enable functions, structs, enums, and traits to work with any data type.
*/

// -------------------------------------
// Example 1: Generic Function
// -------------------------------------

/// A simple generic function that returns either `b` or `c`
/// depending on whether `a` is the character 'a'.
fn generic<T>(a: char, b: T, c: T) -> T {
    if a == 'a' {
        b
    } else {
        c
    }
}

/// Demonstrating the use of the generic function with different types.
pub fn use_of_generic() {
    let a = generic::<f64>('a', 5.5, 1.2);  // Uses f64
    let b = generic::<i32>('a', 5, 1);      // Uses i32
    println!("{}, {}", a, b);
}

// -------------------------------------
// Example 2: Generic Structs
// -------------------------------------

/// A generic struct with two type parameters.
struct S<T1, T2> {
    c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}

// Usage Example:
// let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02 };

/// A tuple struct version using generics.
struct SE<T1, T2>(char, T1, T1, T2);

// Usage Example:
// let _se = SE::<u16, f32>('a', 34, 782, 0.02);

// -------------------------------------
// Example 3: Generic Enum
// -------------------------------------

/// A generic enum that represents possible results.
enum _Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode),
    Uncertainly,
}

/// Demonstrates the use of the generic enum.
fn _enum_generic() {
    let mut _a = _Result1::Success::<i32, u16>(12);
    let _b = &mut _a;
    *_b = _Result1::Uncertainly;
}

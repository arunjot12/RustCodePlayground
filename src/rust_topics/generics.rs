// Example of Generics

fn generic<T>(a: char, b: T, c: T) -> T {
    if a == 'a' {
        b
    } else {
        c
    }
}

pub fn use_of_generic() {
    let a = generic::<f64>('a', 5.5, 1.2);
    let b = generic::<i32>('a', 5, 1);
    println!("{},{}", a, b);
}

// Generic Struct
struct S<T1, T2> {
    c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}
//let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02 };
struct SE<T1, T2>(char, T1, T1, T2);
//let _se = SE::<u16, f32>('a', 34, 782, 0.02);

enum _Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode),
    Uncertainly,
}

fn _enum_generic() {
    let mut _a = _Result1::Success::<i32, u16>(12);
    let _b = &mut _a;
    *_b = _Result1::Uncertainly
}

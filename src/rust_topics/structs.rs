// =====================================
//         RUST REFERENCES & STRUCTS
// =====================================

/*
This file includes examples of:
- Using references (mutable and immutable)
- Defining and using structs
- Creating and matching enums
- Constants
*/

// -------------------------------------
// Example 1: Mutating Array via Reference
// -------------------------------------

#[cfg(feature = "reference")]
fn reference_array_example() {
    let mut arr = [1, -5, 12, -98];
    double_negative(&mut arr);
    println!("{:?}", arr); // Output: [1, -10, 12, -196]
}

/// Doubles each negative number in the array using a mutable reference.
fn _double_negative(a: &mut [i16; 4]) {
    for i in a {
        if *i < 0 {
            *i *= 2;
        }
    }
}

// -------------------------------------
// Example 2: Mutable Reference to a Variable
// -------------------------------------

pub fn reference() {
    let mut x = 10;
    let y = &mut x;

    println!("Before: {}", y);
    *y += 5;
    println!("After: {}", x);
}

// -------------------------------------
// Example 3: Basic Struct
// -------------------------------------

#[allow(unused)]
struct Employee {
    name: String,
    atr: u32,
}

#[allow(unused)]
fn show_data() {
    let data = Employee {
        name: "Tom".to_string(),
        atr: 592,
    };
    // Can access fields: data.name, data.atr
}

// -------------------------------------
// Example 4: Enum and Constant with Struct
// -------------------------------------

#[allow(unused)]
const MAXIMUM_POWER: u16 = 1000;

#[allow(unused)]
enum VehicleKind {
    Car,
    Bike,
    Truck,
}

#[allow(unused)]
struct VehicleData {
    kind: VehicleKind,
    registration_year: u16,
    registration_month: u8,
    power: u16,
}

#[allow(unused)]
fn vehicle() {
    let vehicle = VehicleData {
        kind: VehicleKind::Car,
        registration_year: 2003,
        registration_month: 11,
        power: 120,
    };

    if vehicle.power > MAXIMUM_POWER {
        println!("Too powerful");
    } else {
        println!("Power is within limit.");
    }
}

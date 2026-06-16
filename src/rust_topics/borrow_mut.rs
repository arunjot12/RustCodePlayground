// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

use std::cell::RefCell;

fn ipsa() {
    let a = RefCell::new("Arunjot");
    let mut c = a.borrow_mut();
    *c = "Gupta oye";
    println!("The a is {:?}",a);
}

fn amit(){
    let a = RefCell::new("Hippo Ipsa");
    *a.borrow_mut() = "UP";
    println!("A is {:?}",a);
}
// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message
use std::rc::Rc;

fn rc_pointer() {
 let a = Rc::new(5);
println!("{}", Rc::strong_count(&a));

let b = Rc::clone(&a);
println!("{}", Rc::strong_count(&a));
 
}
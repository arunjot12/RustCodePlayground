#[allow(unused)]
use std::mem::size_of;

pub fn size(){
    let _a: u8 = 255;
    let _b: i8 = -128;
    println!("Size of a: {} bytes", size_of::<u8>());
    println!("Size of b: {} bytes", size_of::<i8>());
    println!("Size of a: {} bytes", size_of::<usize>());
    println!("Size of b: {} bytes", size_of::<usize>());
}
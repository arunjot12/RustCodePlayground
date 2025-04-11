// Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message

fn _refernce() {
    let mut arr = [1,-5,12,-98];
    _double_negative(&mut arr);
    println!("{:?}",arr)
 }
 
 fn _double_negative(a:&mut[i16;4]) {
     for i in a {
         if *i < 0 {
             *i *= 2;
         }
     }
 }
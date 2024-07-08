use std::vec;

pub fn reverse(){
    let mut number = 199;
    let mut vec2= Vec::new();
   while number != 0 {
    let n = number%10;
    vec2.push(n);
    number/=10;
   } 
   let new_number :i32= vec2.iter().map(|&x|x.to_string()).collect::<String>().parse().expect("new");
   println!("{}",new_number);
}
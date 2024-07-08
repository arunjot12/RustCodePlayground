pub fn reverse(){
    println!("Please enter the number");
    let mut number = String::new();
    std::io::stdin() .read_line(&mut number).expect("Failed to read line");
    let mut number: u32 =  number.trim().parse().expect("REASON");

    let mut vec2= Vec::new();
   while number != 0 {
    let n = number%10;
    vec2.push(n);
    number/=10;
   } 
   let new_number :i32= vec2.iter().map(|&x|x.to_string()).collect::<String>().parse().expect("new");
   println!("{}",new_number);
}
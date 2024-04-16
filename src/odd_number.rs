use std::io;
/// 
pub fn odd_number(){
    println!("Please enter the number till you want the print the odd number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number :u32 = input.trim().parse().expect("Please type a number!");

    for mut i in 0..number{
        if ! i%2 == 0 {
            println!("the number is {}",i);
           i+=1;
        }


    }


}
use std::io;

pub fn calculate() {
    println!("Enter the number ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let number: u32 = number.trim().parse().expect("not a number");

    println!("Choose the calculation method");
    println!("Addition");
    println!("Substraction");
    println!("Multiplication");
    println!("Division");
    
    match "" {
        "Addition" => addition(number),
        "Substraction" => substraction(number),
        "Multiplication" => multiplication(number),
        "Division" => division(number),
        _ => println!("Thank you"),
    }

    fn addition(number:u32) {
        let first_number = number;
        let second_number =  input_number();
        let total =  first_number + second_number;
        println!("total numberis {}",total);
    }
    fn substraction(number:u32) {
        let first_number = number;
        let second_number =  input_number();
        let total =  first_number + second_number;
        println!("total number is {}",total);
    }
    fn multiplication(number:u32) {
        let first_number = number;
        let second_number =  input_number();
        let total =  first_number + second_number;
        println!("total number is {}",total);
    }
    fn division(number:u32) {
        let first_number = number;
        let second_number =  input_number();
        let total =  first_number + second_number;
        println!("total number is {}",total);
    }

    fn input_number() -> u32 {
        let mut number = String::new();
        io::stdin().read_line(&mut number).unwrap();
        let number: u32 = number.trim().parse().expect("not a number");
        number
    }
}

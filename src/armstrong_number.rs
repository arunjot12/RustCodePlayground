pub fn armstrong_number() {
    let original_number = 153;
    let mut number = original_number;
    let mut result = 0;
    
    while number !=0 
    {
         let remainder = number % 10;
         result += remainder * remainder * remainder;
         number /= 10;
    }
    
    if result == original_number
    {
        print!("is a armstrong number");
    }
    else
    {
        print!("not a armstrong number");
    }
 }
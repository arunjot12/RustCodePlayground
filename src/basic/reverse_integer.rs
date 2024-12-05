// Program to reverse the integer
pub fn reverse_integer(){
    let mut number = 65;
    let mut reversed = 0;
    while number > 0{
        let digit = number % 10;
        reversed = reversed * 10 + digit;
        number /= 10;
    }
    println!("The reverse integer is {}",reversed)

}
pub fn palindrome(){
    let number = 122;
    let rev :u32= number.to_string().chars().rev() .collect::<String>().parse().expect("not a number");
    if rev == number{
        println!("is a palindrome");
    }
    else{
        println!("not a palindrome")
    }
}
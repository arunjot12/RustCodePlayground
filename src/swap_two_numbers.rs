pub fn swap_numbers(){
    let mut number_one = 1;
    let mut number_second = 3;
    println!("number_one {},number_two {}",number_one,number_second);
    let swap_second = number_one;
    number_one = number_second;
    number_second = swap_second;
    println!("number_one {},number_two {}",number_one,number_second);

}
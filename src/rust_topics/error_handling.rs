// Error handling

use std::fmt::format;

fn divide(numerator:u32,denominator:u32) -> Result<u32,String>{
    if denominator == 0 {
        Err(format!("Number is zero"))
    }
    else{
        Ok(numerator/denominator)
    }
}

fn show_divide(num:u32,den:u32){
    match divide(num,den) {
        Ok(value)=> println!("{}/{} = {}",num,den,value),
        Err(msg)=>  println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}

fn factorial() {
    let mut original_number = 5;
    let mut mult = 0;
    let mut result = 1;
   while (original_number!=0 ){
     let new_number = original_number -1;  
     result = original_number * result;
     original_number-=1;
      println!("{}",result);
    }
    println!("{}",result);
 }
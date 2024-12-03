pub fn largest_string_array(){
    let arr = ["arunjot","singh","antier","blockchain"];
    let mut largest_array = arr[0];
    for i in arr{
        if largest_array.len() < i.len(){
            largest_array = i;
        }
    } 
    println!("Largest array is {}",largest_array);

}
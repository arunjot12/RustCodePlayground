// Largest array in string using len funtion
pub fn largest_array(){
    let arr = ["arunjot","singh","antier","blockchain"];
    let mut largest_array = arr[0];
    for i in arr{
        if largest_array.len() < i.len(){
            largest_array = i;
        }
    } 
    println!("Largest array is {}",largest_array);

}

// Largest array in string without using len function
pub fn largest_string_array(){
    let arr = ["arunjot","singh","antier","blockchain"];
    let mut largest_array = arr[0];
    let mut largest_length = 0;
    for string in arr{
        let mut current_length = 0; 
        for _ in string.chars(){
            current_length+=1;
        }
        if current_length > largest_length{
            largest_array = string;
            largest_length = current_length;
        }
    }
    println!("The largest string is '{}', with length {}", largest_array, largest_length);
}
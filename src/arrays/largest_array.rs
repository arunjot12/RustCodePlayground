/// Largest number in array

pub fn largest_number_array(){
    let arr :[i32;5] = [1,2,5,4,3];
    
    let mut first_arr = arr[0];
    for number in arr{
        if number > first_arr{
            first_arr = number;
        }
    }
    println!("the largest element of array is {}",first_arr);
}
pub fn max_min_array(){
    let arr :[i32;5] = [1,2,3,4,5];
    let mut max_arr =arr[0];
    let mut min_arr = arr[0];
    for i in arr{
        if max_arr < i {
            max_arr = i;
        }
        if min_arr > i{
            min_arr = i;
        }
    }
    println!("The maximum number is {}, The minimum number is {}",max_arr,min_arr)
}
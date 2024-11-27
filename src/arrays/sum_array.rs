pub fn sum_of_array(){
    let array =[1,2,3,4,5];
    let mut sum = 0;
    for i in array{
        sum= sum+i;
    }
    println!("Total of array is {}",sum);
}
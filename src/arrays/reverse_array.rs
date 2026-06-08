// Reverse of the array without using in-built function
pub fn reverse() {
    let mut arr = [1, 2, 3, 4, 5]; 
    let length = arr.len();
    
    for i in 0..length / 2 {
        let temp = arr[i];
        arr[i] = arr[length - 1 - i];
        arr[length - 1 - i] = temp;
    }
    
    println!("{:?}", arr);
 }

 // Reverse array using in-built function
pub fn reverse_array(){
    let arr = [1,2,3,4,5];
    // arr.reverse();
    let rev: Vec<_> = arr.iter().rev().collect();
    println!("{:?}",rev);

} 
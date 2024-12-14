pub fn reverse_array() {
    let mut arr = [1, 2, 3, 4, 5]; 
    let length = arr.len();
    
    for i in 0..length / 2 {
        let temp = arr[i];
        arr[i] = arr[length - 1 - i];
        arr[length - 1 - i] = temp;
    }
    
    println!("{:?}", arr);
 }
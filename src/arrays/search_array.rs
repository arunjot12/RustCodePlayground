pub fn search_array() {
    println!("Enter the number of elements for 5 size");
    let mut arr: [u32; 5] = [0; 5];
    for i in 0..5 {
        let mut new = String::new();
        std::io::stdin().read_line(&mut new).unwrap();
        let new: u32 = new.trim().parse().expect("not a number");
        arr[i] = new;
    }
    println!("Enter the index number that you want to check for the value");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).unwrap();
    let index: u32 = index.trim().parse().expect("not a number");

    for i in arr {
        if i == index  {
            print!("The number of index {:?}", arr[index as usize]);
        }
    }
}

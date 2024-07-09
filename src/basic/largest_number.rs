
pub fn largest_number(){
    println!("Please enter the number");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    let number :u32= number.trim().parse().expect("not a number");
    let mut store :Vec<u32>= Vec::new();
    for _i in 0..number{
        let mut element = String::new();
        std::io::stdin().read_line(&mut element).unwrap();
        let element:u32 = element.trim().parse().expect("not a number");
        store.push(element)
    }
    let mut data = store[0];
    for i in store.iter(){
        if *i > data {
            data = *i
        }
    }
    println!("The largest number is {}",data);
}
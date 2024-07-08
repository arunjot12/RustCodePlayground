
pub fn largest_number(){
    println!("Please enter the number ");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    let number :u32= number.trim().parse().expect("not a number");
    for i in 0..number{
        let mut element = String::new();
        std::io::stdin().read_line(&mut element).unwrap();
        let element:u32 = element.trim().parse().expect("not a number");
        let mut store :Vec<u32>= Vec::new();
        store.push(element)
    }
}
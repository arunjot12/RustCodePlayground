// Example of Option

fn _option() {
    let mut data = vec![1, 2, 3];
    for _ in 0..5 {
        let data = data.pop();
        match data {
            Some(data) => println!("{}", data),
            None => println!("#"),
        }
    }
}

// Option using unwrap()

pub fn option_unwrap(){
    let data :Option<u32> = None;
    println!("{:?}",data);

    match data {
        Some(value) => println!("{}",value),
        None => {}
    }
}
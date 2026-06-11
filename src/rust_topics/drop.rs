struct CustomData {
    data: String,
}

impl Drop for CustomData {
    fn drop(&mut self) {
        println!("Dropping  {}", self.data)
    }
}

pub fn show_drop() {
    println!("--- Drop Trait Demo ---");
    let _a = CustomData {
        data: String::from("First Instance"),
    };

    let _b = CustomData {
        data: String::from("Second Instance"),
    };

    println!("Variables created. About to end scope...");
}

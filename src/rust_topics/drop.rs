struct CustomData{
    data : String
}

impl Drop for CustomData{
    fn drop(&mut self) {
        println!("Dropping  {}",self.data)
    }
}

fn main(){
    let a = CustomData{
        data: String::from("big boss")
    };

    let b = CustomData{
        data : String::from("hhhhh")
    };

    println!("Boom ")
}
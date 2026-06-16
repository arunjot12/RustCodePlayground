
struct Counter{
    count:i32
}

impl Counter {
    fn new()-> Counter {
        Counter{ count : 0}
    }
}

impl Iterator for Counter{
    type Item = i32;
    fn next(&mut self) -> Option<i32>{
        self.count += 1;

        if self.count < 5{
         return Some(self.count)
        }
        else{
          None
        }
    }
}

fn custom_iterator(){
    let mut a = Counter::new();
    println!("a is {:?}",a.next());
    println!("a is {:?}",a.next());

}


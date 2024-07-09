// Half Pyramid Pattern of Numbers
pub fn simple_pyramid() {
    for i in 1..=5 {
        for _j in 1..=i {
            print!("{}", i);
        }
        println!("");
    }
}

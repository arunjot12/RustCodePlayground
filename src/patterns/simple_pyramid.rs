// Half Pyramid Pattern of Numbers
pub fn simple_pyramid() {
    for i in 1..=5 {
        for _j in 1..=i {
            print!("{}", i);
        }
        println!("");
    }
}

// Reverse Pyramid
pub fn reverse_pyramid() {
    for i in (1..=5).rev() {
        for _j in 1..=i {
            print!("{}", _j);
        }
        println!("");
    }
}
pub fn full_pyramid() {
    for i in 1..=1 {
        for _ in 1..=7 - i {
            print!(" ");
        }
        for _ in 1..(2 * i) {
            print!("{} ", i);
        }
        println!();
    }
}

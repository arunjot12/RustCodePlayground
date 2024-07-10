pub fn full_pyramid() {
    for i in 1..=5 {
        for _ in 1..=5 - i {
            print!(" ");
        }
        for _ in 1..(2 * i) {
            print!("{}", i);
        }
        println!();
    }
}

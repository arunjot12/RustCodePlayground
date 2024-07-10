pub fn full_pyramid() {
    // Start the iterations
    for i in 1..=5 {
        // Print the space in between the numbers
        for _ in 1..=5 - i {
            print!(" ");
        }
        // Print the numbers
        for _ in 0..i {
            print!("{} ", i);
        }

        // Change the line
        println!();
    }
}

pub fn reverse_pyramid() {
  for i in (1..5) {
        for _ in 1..=(5 - i) {
            print!(" ");
        }
        for _ in 1..(2 *i) {
            print!("*");
        }
        println!()
    };
    for i in (1..5).rev() {
        for _ in 1..=(5 - i) {
            print!(" ");
        }
        for _ in 1..(2 *i) {
            print!("*");
        }
        println!()
    };
    

}

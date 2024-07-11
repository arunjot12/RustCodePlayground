pub fn square_pyramid() {
    for i in 1..=5 {
        for k in 1..=5 {
            if i == 1 || i == 5 ||  k==5 {
                print!("* ");
            }
            else if i >=2 && k == 1   {
                print!("* ");
            } 
            else{
                print!("  ");
            }
        }
        println!();
    }
}

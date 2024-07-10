pub fn pyramid_numbers(){
    for i in 1..=5{
        for j in 1..i{
            print!("{}",j);
        }
        println!()
    }
    for i in (1..=4).rev(){
        for j in 1..i{
            print!("{}",j);
        }
        println!()
    }

}
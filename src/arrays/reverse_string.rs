pub fn reverse_string(){
    let string = "arunjot";
    for i in string.chars().rev(){
        println!("{}",i);
    }
}
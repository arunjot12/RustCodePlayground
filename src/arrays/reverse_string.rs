// Reverse String With using in built functions
pub fn reverse_string(){
    let string = "arunjot";
    for i in string.chars().rev(){
        print!("{}",i);
    }
}
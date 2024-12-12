pub fn removing_duplicasy(){
    let mut data = vec![0,0,1,1,1,2,2,3,3,4];
    let mut removed_data:Vec<i32> = vec![];
    for i in data{
        if removed_data.contains(&i){
           data()
        }
        else{
            removed_data.push(i);
        }
    }
    let length = removed_data.len() as i32;
    println!("removed data {:?}",length)

}
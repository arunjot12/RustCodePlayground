use std::collections::Hashmap;
fn hashmap(){
    let mut map = Hashmap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("{:?}", map);
}
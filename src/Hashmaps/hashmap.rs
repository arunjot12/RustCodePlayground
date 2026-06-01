use std::collections::HashMap;

pub fn hashmap() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("{:?}", map);
}

pub fn hashmap_entry() {
    let mut map = HashMap::new();
    map.insert(String::from("Arunjot"), String::from("clear"));
    let check = map.entry(String::from("Arunjot")).or_insert(String::from("default"));
    println!("{:?}", check);
}

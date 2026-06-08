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
    let _check: &mut String = map.entry(String::from("Arunjot")).or_insert(String::from("Pass"));
    let value = map.get(&String::from("Arunjot"));
    println!("{:?}", value);
}

pub fn deference_array(){
    let string = "Hello Brother Waheguru";
    let mut map = HashMap::new();

    for words in string.split_whitespace(){
        let count = map.entry(words).or_insert(0);
         *count += 1;
    }
}
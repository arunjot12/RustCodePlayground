pub fn lifetime() {
    let a = "arunjot";
    let b = "singh";
    let result = longest_string(a, b);
    println!("Longest string: {}", result);
}

pub fn longest_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
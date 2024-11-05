use std::{array, vec};
use ethabi::Token;
use tiny_keccak::{ Hasher, Keccak };

struct Zombie {
    name: String,
    dna: String,
}

static mut ZOMBIES :Vec<Zombie>= vec![];

// Random function to create a zombie
pub unsafe fn create_zombie() {
    println!("Enter the name of the zombie");
    let mut new = String::new();
    std::io::stdin().read_line(&mut new).unwrap();
    let zombie: String = new.trim().parse().expect("not a number");
    let dna = create_random_zombie(zombie.clone());
    println!("the dna for the present zombie is {}", dna);
    let data = Zombie{name:zombie,dna:dna};
    ZOMBIES.push(data);
}

/// Function to create the keccak hash
fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(data);
    keccak.finalize(&mut output);
    output
}

/// Function to create a dna for the zombie
pub fn create_random_zombie(name: String) -> String {
    // Encode the name as the input for the hash function
    let tokens = vec![Token::String(name)];
    let encoded = ethabi::encode(&tokens);

    // Generate a Keccak256 hash of the encoded input
    let hash = keccak256(&encoded);

    // Convert the resulting hash to a hexadecimal string
    let dna = hex::encode(hash);

    // Optionally, truncate the DNA to a specific length
    let truncated_dna = &dna[0..16]; // Example: first 16 characters

    truncated_dna.to_string()
}

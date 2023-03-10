use sha2::{Digest, Sha256};

pub fn print_hash(hash: &[u8; 32]) {
    let mut hex_string = String::new();

    for byte in hash {
        hex_string.push_str(&format!("{:02x}", byte));
    }

    println!("0x{}", hex_string);
}

pub fn hash_data(transaction: Vec<String>) -> [u8; 32] {
    let mut hasher = Sha256::new();

    for item in transaction {
        hasher.update(item);
    }

    let result = hasher.finalize();

    result.into()
}

pub fn calculate_hash(left_hash: &[u8; 32], right_hash: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(left_hash);
        hasher.update(right_hash);
        let result = hasher.finalize();
        result.into()
    }
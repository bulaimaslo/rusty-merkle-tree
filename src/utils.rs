
pub fn print_hash(hash: &[u8; 32]) {
    let mut hex_string = String::new();

    for byte in hash {
        hex_string.push_str(&format!("{:02x}", byte));
    }

    println!("0x{}", hex_string);
}

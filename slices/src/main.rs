fn main() {
    // Real Solana address
    let wallet_address = String::from("7xW2js89TkMz1Lp8GhqA49pL");

    // call our function to abbreviate the wallet address
    let short_address = abbreviate_wallet(&wallet_address);

    println!("Original address: {}", wallet_address);
    println!("Abbreviated address: {}", short_address);
}

fn abbreviate_wallet(address: &str) -> String {
    // we dont copy the string, we just take slices of it
    
    // we take the first 4 characters using a slice
    let first_four = &address[0..4];
    
    // we take the last 4 characters
    // we use .len() to know where it ends
    let length = address.len();
    let last_four = &address[length - 4..length];

    format!("{}...{}", first_four, last_four)
}
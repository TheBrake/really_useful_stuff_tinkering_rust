
//struct AdminRole; // Dont have any data, just a marker for admin privileges.

#[derive(Debug)] // this part writing the code for the struct to be printable with {:?} in println!
struct Pubkey(String); 

#[derive(Debug)]
struct TokenAccount {
    owner: Pubkey,
    balance: u64,
    symbol: String,
}

fn main() {
 
    let send_result: (bool, &str) = (false, "Transfer successful!");

    let wallet_zohar = Pubkey(String::from("Zohar123..."));
    let wallet_dikaios = Pubkey(String::from("Dikaios456..."));

    let mut zohar_account = TokenAccount {
        owner: wallet_zohar,
        balance: 500,
        symbol: String::from("SOL"),
    };

    let dikaios_account = TokenAccount {
        owner: wallet_dikaios,
        balance: 5000,
        symbol: String::from("SOL"),
    };

    println!("\ninitial state of Dikaios's account: {:?}\n", dikaios_account);

    // print initial state of Zohar's account
    println!("initial state of Zohar's account: {:?}", zohar_account);

    if send_result.0 { 
        println!("\nConfirmation: {}", send_result.1);

        zohar_account.balance -= 100;
    } else {
        println!("\nTransfer failed");
    }

    println!("\nfinal state of Zohar's account: {:?}\n", zohar_account);

    // Compare balances and print the account with the most balance
    if zohar_account.balance > dikaios_account.balance {
        let first_words = &zohar_account.owner.0[..5];
        println!("Most balance account: {} | {}", first_words, zohar_account.symbol);
    } else {
        let first_words = &dikaios_account.owner.0[..5];
        println!("Most balance account: {} | {}", first_words, dikaios_account.symbol);
    }   

}

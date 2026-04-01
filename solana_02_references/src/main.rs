struct SolanaAccount {
    pubkey: String,
    lamports: u64,
    is_signer: bool,
}

fn main() {

    let mut user_account = SolanaAccount {
        pubkey: String::from("7xW..."),
        lamports: 1000,
        is_signer: true,

    };

    print_account_info(&user_account);

    match process_payment(&mut user_account, 500) {
        Ok(_) => println!("Pago procesado. Nuevo balance: {}", user_account.lamports),
        Err(e) => println!("Error: {}", e),
    }
}


fn print_account_info(account: &SolanaAccount) {
    println!("Pubkey: {}, Balance: {}", account.pubkey, account.lamports);
    
}

fn process_payment(account: &mut SolanaAccount, amount: u64) -> Result<(), String> {
    
    if account.is_signer {
        account.lamports = account.lamports.checked_sub(amount)
        .ok_or(("insufficient funds").to_string())?;
        Ok(())
    } else {
        Err(("only signer can process payment").to_string())
    }
}
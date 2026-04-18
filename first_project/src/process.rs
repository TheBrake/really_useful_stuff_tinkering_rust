use crate::state::UserAccount;
//Deposit

fn add_deposit(account: &mut UserAccount, amount: u64) {
    let new_balance = account
        .balance
        .checked_add(amount)
        .expect("Overflow al añadir el depósito");
    account.balance = new_balance;
    println!("Depósito exitoso. Nuevo balance: {}", account.balance);
}

//Withdraw

fn add_withdraw(account: &mut UserAccount, amount: u64) {
    if let Some(new_balance) = account.balance.checked_sub(amount) {
        account.balance = new_balance;
        println!("Retiro exitoso. Nuevo balance: {}", account.balance);
    } else {
        eprintln!("Fondos insuficientes");
    }
}

//viewbalance

fn view_balance(account: &UserAccount) {
    println!("Saldo actual: {}", account.balance);
}

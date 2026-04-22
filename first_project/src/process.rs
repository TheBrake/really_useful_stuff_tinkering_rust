use crate::state::UserAccount;
use crate::BankInstruction;
//Deposit

fn add_deposit(account: &mut UserAccount, amount: u64) {
    let new_balance = account
        .balance
        .checked_add(amount)
        .expect("Overflow al añadir el depósito");
    account.balance = new_balance;
    println!("\nDepósito exitoso. Nuevo balance: {}", account.balance);
}

//Withdraw

fn add_withdraw(account: &mut UserAccount, amount: u64) {
    if let Some(new_balance) = account.balance.checked_sub(amount) {
        account.balance = new_balance;
        println!("\nRetiro exitoso. Nuevo balance: {}", account.balance);
    } else {
        eprintln!("\nFondos insuficientes");
    }
}

//viewbalance

fn view_balance(account: &UserAccount) {
    println!("\nSaldo actual: {}", account.balance);
}

pub fn process(instruccion: BankInstruction, account: &mut UserAccount){
    match instruccion {
        BankInstruction::Deposit(amount) => add_deposit(account, amount),
        BankInstruction::Withdraw(amount) => add_withdraw(account, amount),
        BankInstruction::ViewBalance => view_balance(account),
    }
}

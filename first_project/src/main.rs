use crate::state::UserAccount;
use std::io;
pub mod process;
pub mod state;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BankInstruction {
    Deposit(u64),
    Withdraw(u64),
    ViewBalance,
}

fn main() {
    println!("\nMenú de Opciones = 1) Depostar, 2) Retirar, 3) Ver saldo");

    println!("\nPor favor Seleccione su Opción");

    let result: u64 = loop {
        let mut decition = String::new();
        io::stdin()
            .read_line(&mut decition)
            .expect("Error en la opcion");

        let _decition: u64 = match decition.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("\nSu opcion elegida es {result}");

    let mut zohar = UserAccount {
        owner: state::Pubkey(String::from("Zohar123...")),
        balance: 500,
    };
    let opcion = validar_opcion(&result);

    // llamamos al proceso.
    if let Some(instruccion) = opcion {
        process::process(instruccion, &mut zohar);
    };

    
}

fn validar_opcion(valid: &u64) -> Option<BankInstruction> {
    
    //ahora el match que valida.
    match valid {
        1 => {
            println!("\nMonto a depositar: ↓");

            fn read_amount() -> u64 {
                loop {
                    let mut amount_str = String::new();
                    io::stdin()
                        .read_line(&mut amount_str)
                        .expect("Error en la opcion");

                    match amount_str.trim().parse() {
                        Ok(num) => return num,
                        Err(_) => continue,
                    };
                }
            }
            return Some(BankInstruction::Deposit(read_amount()));
        }
        2 => {
            println!("\nMonto a retirar: ↓");

            fn read_withdraw() -> u64 {
                loop {
                    let mut withdraw_str = String::new();
                    io::stdin()
                        .read_line(&mut withdraw_str)
                        .expect("Error en la opcion");

                    match withdraw_str.trim().parse() {
                        Ok(num) => return num,
                        Err(_) => continue,
                    };
                }
            }
            return Some(BankInstruction::Withdraw(read_withdraw()));
        }

        3 => {
            return Some(BankInstruction::ViewBalance);
        }
        _ => None,
    }

}

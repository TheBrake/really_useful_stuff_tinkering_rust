use std::io;
pub mod state;
pub mod process;

#[allow(dead_code)]
#[derive(Debug)]
enum BankInstruction {
    Deposit(u64),
    Withdraw(u64),
    ViewBalance,
}

fn main() {
    println!("Menú de Opciones = 1) Depostar, 2) Retirar, 3) Ver saldo");

    println!("Por favor Seleccione su Opción");

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

    println!("Su opcion elegida es {result}");

    let read_decition = validar_opcion(&result);
}

fn validar_opcion(valid: &u64) -> Option<BankInstruction> {
    //ahora el match que valida.
    match valid {
        1 => {
            //reducir codigo repetido, crear función para leer cantidad a depositar o retirar
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
            //no me ayudes, lo hare solo en este caso, quiero practicar.
            println!("Monto a retirar");

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
            return Some(BankInstruction::Withdraw(read_withdraw()));
        }
    }

        3 => {
            println!("Aqui se verá el balance");
            return Some(BankInstruction::ViewBalance);
        }
        _ => None,
    }
}

/*
main.rs
  │
  ├── Muestra menú al usuario (Depositar / Retirar / Ver Saldo) = Completado
  ├── Lee la opción = Completado
  ├── Crea un BankInstruction según la opción = Completado
  └── Llama a processor::process(instruction, &mut account) = FALTA POR HACER
        │
        ├── Si Deposit(amount)  → account.balance += amount
        ├── Si Withdraw(amount) → verifica saldo, luego account.balance -= amount
        └── Si ViewBalance      → imprime account.balance

*/

// Qué es UserAccount: owner_address (String), balance (u64)
// Qué es BankInstruction: Deposit(u64), Withdraw(u64), ViewBalance
// Qué hace processor::process: recibe instrucción + cuenta mutable, ejecuta
// Qué hace state: solo define las estructuras, no tiene lógica

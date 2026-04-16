use std::io;

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
            let ingreso: u64 = loop {
                let mut añadir = String::new();
                io::stdin()
                    .read_line(&mut añadir)
                    .expect("Error en la opcion");

                let _añadir: u64 = match añadir.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => continue,
                };
            };
            return Some(BankInstruction::Deposit(ingreso));
        }
        2 => {
            let retiro: u64 = loop {
                let mut restar = String::new();
                io::stdin()
                    .read_line(&mut restar)
                    .expect("Error en la opcion");

                let _restar: u64 = match restar.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => continue,
                };
            };
            return Some(BankInstruction::Withdraw(retiro));
        }
        3 => {
            println!("Aqui se verá el balance");
            return Some(BankInstruction::ViewBalance);
        }
        _ => None
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

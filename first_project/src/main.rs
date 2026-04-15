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
    //let read_decition = process(&result);
}


/*
main.rs
  │
  ├── Muestra menú al usuario (Depositar / Retirar / Ver Saldo) = Completado
  ├── Lee la opción = Completado
  ├── Crea un BankInstruction según la opción
  └── Llama a processor::process(instruction, &mut account)
        │
        ├── Si Deposit(amount)  → account.balance += amount
        ├── Si Withdraw(amount) → verifica saldo, luego account.balance -= amount
        └── Si ViewBalance      → imprime account.balance

*/

// Qué es UserAccount: owner_address (String), balance (u64)
// Qué es BankInstruction: Deposit(u64), Withdraw(u64), ViewBalance
// Qué hace processor::process: recibe instrucción + cuenta mutable, ejecuta
// Qué hace state: solo define las estructuras, no tiene lógica
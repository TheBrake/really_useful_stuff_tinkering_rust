fn main() {
    let mut balance: u64 = 200_000_000; 
    let deposito: u64 = 100_000_000;


    // Usamos if let para manejar el posible error sin que el programa explote
    if let Some(nuevo_ingreso) = balance.checked_add(deposito) {
        balance = nuevo_ingreso;
        println!("Se añadieron {} SOL correctamente. Nuevo saldo: {}", deposito, balance);
    }

    let retiro: u64 = 150_000_000;
    let confirmar_retiro = true; 

    if confirmar_retiro {
        // En lugar de expect, manejamos el error con un mensaje personalizado
        match balance.checked_sub(retiro) {
            Some(nuevo_saldo) => {
                balance = nuevo_saldo;
                println!("Retiro exitoso. Saldo: {}", balance);

                // Cálculo de intereses (10%)
                let intereses = balance.checked_mul(10)
                    .and_then(|m| m.checked_div(100))
                    .expect("Error crítico en cálculo de intereses");

                balance += intereses;
                println!("Tu balance final con intereses es: {}", balance);
            }
            None => println!("Error: Fondos insuficientes para retirar {}", retiro),
        }
    } else {
        println!("Operación cancelada por el usuario.");
    }
}
use std::io;
use std::cmp::Ordering;
use rand::RngExt;

fn main() {
    println!("Adivina el numero");

    let secret_number = rand::rng().random_range(1..=100);

    println!("El numero secreto es: {secret_number}");

    loop {
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("error en leer la linea");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
        
    println!("Por favor ingresá tu numero");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Demasiado bajo!"),
        Ordering::Greater => println!("Demasiado alto!"),
        Ordering::Equal => {
            println!("Ganaste!");
            break;
        }
    }
}

}
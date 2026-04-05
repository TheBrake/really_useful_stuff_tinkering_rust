#[derive(Debug)]
struct Inventory {
    coins: u32,
    max_capacity: u32,
}

impl Inventory {

    fn new(capacity: u32) -> Self {
        Self { 
            coins: 0, 
            max_capacity: capacity }
    }

    // 2. Lógica de validación con &&
    fn can_add(&self, coins: u32) -> bool {
        self.coins >= 0 && self.coins + coins <= self.max_capacity
    }

    // 3. Método para modificar datos
    fn add_coins(&mut self, coins: u32) {

        if self.can_add(coins) {
            self.coins += coins;
            println!("¡Monedas añadidas! Total: {}", self.coins);
        } else {
            println!("Error: No caben o cantidad inválida.");
        }
    }

    fn is_full(&self) -> bool {
        self.coins >= self.max_capacity
    }
}

fn main() {
    let mut my_chest = Inventory::new(100);

    my_chest.add_coins(30); // Debería funcionar
    my_chest.add_coins(71); // Esto debería fallar por el límite

}
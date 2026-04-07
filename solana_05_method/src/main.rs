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

    
    fn can_add(&self, coins: u32) -> bool {
        self.coins + coins <= self.max_capacity
    }

    
    fn add_coins(&mut self, coins: u32) {

        if self.can_add(coins) {
            self.coins += coins;
            println!("¡Coins added! Total: {}", self.coins);
        } else {
            println!("Error: No space available or invalid amount.");
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

    if my_chest.is_full() {
        println!("Sorry, your chest is full.");
    } else {
        println!("Your remaining capacity is: {}", my_chest.max_capacity - my_chest.coins);
    }

}
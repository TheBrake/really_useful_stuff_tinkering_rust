enum OrderType {
    Market,
    Limit(f64)
}

enum OrderState {
    Open,
    PartiallyFilled(f64),
    Filled
}


struct Order {
    id: u32,
    order_type: OrderType,
    state: OrderState,
    payout_address: Option<String>, // si no hay direccion de payout, es None
}

fn main() {

    let my_order = Order {
        id: 1,
        order_type: OrderType::Limit(150.5),
        state: OrderState::Open,
        payout_address: None,
    };

    
    match my_order.order_type {
        OrderType::Market => println!("Ejecutando orden a precio mercado"),
        OrderType::Limit(price) => println!("Esperando precio de {}", price),

    }

    println!("Destino: {}", my_order.payout_address.unwrap_or("Dirección por defecto".into()))
}

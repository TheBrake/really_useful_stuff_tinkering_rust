fn main() {
    let fahrenheit: [f32; 5] = [32.0, 68.0, 100.0, 212.0, 451.0];

    for f in fahrenheit.iter().rev() {
        let c: f32 = (f - 32.0) * 5.0 / 9.0;
        println!("La conversion a sido exitosa de {f:.2}°F a {c:.2}°C");
    }

}

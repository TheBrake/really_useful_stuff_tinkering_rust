fn main() {
    let mut a = 0;
    let mut b = 1;
    
    for _fibo in 0..10 {
        let fibo = a + b;
        a = b;
        b = fibo;
        println!("The current value is:{} ", fibo);
    }
}

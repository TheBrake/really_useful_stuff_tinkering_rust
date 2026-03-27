fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    let z = 5;
    let z = z + 1; // Shadowing allows us to reuse the same variable name, but with a new value.
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    let spaces = "   ";
    let spaces= spaces.len();
    println!("The value of spaces is: {spaces}");
}
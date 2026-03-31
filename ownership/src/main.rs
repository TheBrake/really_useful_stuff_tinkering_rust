fn main() {
 let user_signature = String::from("0x123456789abcdef"); 

 let lend_signature = user_signature.clone();
 validate_signature(lend_signature);

 print!("\nThe original signature is still valid: {user_signature}\n");
}

fn validate_signature(signature: String) {
    print!("\nValidando firma: {signature}\n");
}

// I read the rust book cap 4.1, and I understand the concept of ownership and moves.
//In this case, i use the clone method to create a copy of the string. 
// My primary goal is for the original variable to retain ownership.
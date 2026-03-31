fn main() {
    let phrases = [
        "A partridge in a pear tree.", "Two turtle doves", "Three French hens",
        "Four calling birds", "Five golden rings", "Six geese a-laying",
        "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing",
        "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"
    ];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for d in 0..12 {
        println!("\n On the {} day of Christmas, my true love gave to me:", days[d]);

        for c in (0..=d).rev() {
            if d > 0 && c == 0 { print!("And "); }
            println!("{}", phrases[c]);
        }
    }
}
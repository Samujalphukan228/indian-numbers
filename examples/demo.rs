use indian_numbers::{format, to_words};

fn main() {
    let amount = 123456789;

    println!("Original Number : {}", amount);
    println!("Indian Format   : {}", format(amount));
    println!("In Words        : {}", to_words(amount));
    println!("Price           : ₹{} only", format(98765432));
}
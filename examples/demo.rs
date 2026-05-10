use indian_numbers::{format, to_rupees, to_words};

fn main() {
    let amount = 123456789;

    println!("Original Number   : {}", amount);
    println!("Indian Format     : {}", format(amount));
    println!("In Words          : {}", to_words(amount));
    println!("Rupees Format     : {}", to_rupees(amount));
    println!();
    
    // More examples
    println!("Price Example     : {}", to_rupees(98765432));
    println!("Small Amount      : {}", to_rupees(499));
    println!("Negative Amount   : {}", to_rupees(-25000));
}
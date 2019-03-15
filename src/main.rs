use std::io;

fn main() {
    println!("Hello, world!");
    
    // `::` denotes that a particular method is an
    // ASSOCIATED FUNCTION that is implemented on the
    // TYPE, not the instance of that type itself.
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)          // `&` indicates that we're passing a reference, but
        .expect("failed to read line"); // like values, references are immutable by default.
                                        // TODO why does this need to be mutable though?

    println!("you guessed: {}", guess);
}

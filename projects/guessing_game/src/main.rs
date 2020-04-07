use std::io;  //Including a headerfile 

//Main function 
fn main() {
    println!("Hello, world!");

    println!("Guess Name!");
    println!("Please input your guess. ");

    let mut guess = String::new();

    io::stdin().read_line(& mut guess).expect("Failed to readline");

    println!("Your Guess {}",guess);
}

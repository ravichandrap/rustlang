use std::io;
// use std::string;

fn main() {
    println!("Hello, world!");
    // const USER_NAME:str = String::from("rc.reddy@outlook.com");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

}

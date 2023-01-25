// importing rand liibrary to generate random number 
// importing Ordering library to check greater smaller or equal to 
// importing io for input and output operations
use rand::Rng;
use std::cmp::Ordering;
use std::io;

 

fn main() {
   
    println!("Guess the number!");
// generates a random number 
    let secret_number = rand::thread_rng().gen_range(1..=100);

  // an infinite loop that ends once you win 
    loop{

println!("please enter your guess numbefer");
// creates a new instance of type String 
let mut guess = String::new();

println!("Your Guess is {guess}");
// fo rtaking input 
// expect for reuslt of Err
// read_line reads the line 
io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
//
//trim trims any /n or spaces 
//parse checks the string with u32 
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
// match checkc for patterns 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
}
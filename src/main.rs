// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

mod tuples;
mod arrays;
mod vectors;

fn main() {
    /*println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1,100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guest.");

    let mut guess = String::new();

    
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    let no_mutable = "teste";
    println!("print variable not mutable: {}", no_mutable);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed {} and secrect number is {}", guess, secret_number);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too BIG"),
        Ordering::Equal => println!("You WIN!!!!"),
    }

    let mut string = String::new();

    println!("Length: {}", string.len());

    // Push a char
    string.push('W');

    // Push a string
    string.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", string.capacity());

    println!("{}", string);*/

    tuples::run();
    arrays::run();
    vectors::run();
}

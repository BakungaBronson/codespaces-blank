use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1, 11);
    println!("Guess the number!");
    println!("Your secret number 🤐: {}", secret_number);
    loop {
        let mut guess = String::new();
        println!("Enter your number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("You entered: {}", guess);
        let guess: u32 = guess.trim().parse().expect("Not a valid number!");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Equal!");
                break;
            },
            Ordering::Less => println!("Less!"),
            Ordering::Greater => println!("Greater!"),
        }
    }
}

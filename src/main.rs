use std::io;    // prelude
use rand::Rng;  // tarit
use std::cmp::Ordering;    // enum

fn main(){
    println!("Let's do a guessing game!");
    loop {
        println!("Guess a number:");
        // declare a mutable varaible
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid input!");
        println!("The number you enter is: {}", guess);
        // generate a target number using Rnd trait
        let target = rand::thread_rng().gen_range(1..101);
        // convert guess to u32 to compare with target number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        // match case
        match guess.cmp(& target) {
            Ordering::Greater => println!("Too Large!"),
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            }
        }        
    }
}

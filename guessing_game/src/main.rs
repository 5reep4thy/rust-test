extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 50);
    let mut guess = String::new();
    loop {
        println!("Guess a number");
        guess.clear();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("Inputted {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(nums) => nums,
            Err(_) => continue
        };
        match guess.cmp(&secret_num) { 
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
        
    }
    println!("You guessed {}", guess);

}

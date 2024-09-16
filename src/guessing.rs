use rand::Rng;
use std::cmp::Ordering;
use colored::*; 
use std::io;


pub fn guessing_game(){

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempt = 5;

    println!("you have 5 attempt");

    loop{ 
        let mut guess = String::new();

       if attempt > 0 {
        attempt = attempt -1; 
        println!("Enter your guess");
        io::stdin()
        .read_line(&mut guess)
        .expect("Did't expect this format");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Greater => println!("{}", "Greater than the value".red()),
            Ordering::Less => println!("{}", "Lesser than the value".red()),
            Ordering::Equal => {
                println!("{}", "Yay, you won!".green());
                break;
            }
        }
       } else {
            println!("Out of luck");
            break;
       }
      
    }
}
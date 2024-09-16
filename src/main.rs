use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*; 
fn main() {

    let s1 = String::from("Name of you");
    let lenc = get_string_len(s1);
    println!("the length {}", lenc);

    guessing_game();
    
}

// Function to compute Fibonacci number
// fn fibonacci(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     }

//     let mut a = 0;
//     let mut b = 1;
//     let mut fib = 0;

//     for _ in 2..=n {
//         fib = a + b;
//         a = b;
//         b = fib;
//     }

//     fib
// }

fn get_string_len(name: String) -> usize {

    let lenc = name.len();
    lenc
}

fn guessing_game(){

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
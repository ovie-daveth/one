mod guessing;
mod chpone;
mod ownership;

fn main() {

    // let s1 = String::from("Name of you");
    // let lenc = get_string_len(s1);
    // println!("the length {}", lenc);

    // guessing::guessing_game();
    // chpone::chpone();
    ownership::owner();
    
}

// pub fn get_string_len(name: String) -> usize {

//     let lenc = name.len();
//     lenc
// }

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


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let lower_limit = 1;
    let upper_limit = 101;
    let secret_number = rand::thread_rng().gen_range(lower_limit, upper_limit);

    loop {
        println!("guess a number between {} and {}", lower_limit, upper_limit - 1);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you didn't enter a number. try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => break,
        }
    }
    println!("you win");
}

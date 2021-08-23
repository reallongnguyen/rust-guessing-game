use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");
    let secret_num = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess in range (1..=10)");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess is invalid\n");
                continue;
            }
        };
        println!("Your guess: {}", guess);

        if guess < 1 || guess > 10 {
            println!("Your guess is not in range (1..=10)\n");
            continue;
        }

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }

        println!("");
    }
}

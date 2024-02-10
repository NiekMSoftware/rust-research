use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // run the  loop
    loop {
        println!("Please enter your guess.");
        eprint!("> ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        // parse string value to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nERR: \nOnly accepting numbers!\n");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess: {guess}, is too small!"),
            Ordering::Greater => println!("Your guess: {guess}, is too big!"),
            Ordering::Equal => {
                println!("Your guess: {guess}, was correct! Good job!");
                break;
            }
        }
    }

    // display final message
    println!("Ending application! \nSee you later!~\n")
}

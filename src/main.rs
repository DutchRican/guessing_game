use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn instructions(max: i32) {
    println!("Guess the number!");
    println!("I have selected a number between 1 and 100.");
    println!("You get a total of {} guesses.", max);
}

fn main() {
    const MAX_GUESSES: i32 = 7;
    instructions(MAX_GUESSES);

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut counter = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);
        counter += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! You have {} guesses left", MAX_GUESSES - counter),
            Ordering::Greater => println!("Too big! You have {} guesses left", MAX_GUESSES - counter),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        if counter >= MAX_GUESSES {
            println!("You lost, you didn't get the number :(");
            println!("It was the number: {}", secret_number);
            break;
        }
    }
}

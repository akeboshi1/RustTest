use std::io;
use rand::Rng; // 0.8.0

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let random_boolean = rand::random::<char>();
    let num = rand::thread_rng().gen_range(0..100);
    println!("{}", num);
    println!("{}",random_boolean);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

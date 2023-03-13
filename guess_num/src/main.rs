use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret num is :{secret_num}");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(& mut  guess).expect("Failed to read line");
    let guess : u32 = guess.trim().parse().expect("Please type a number");
    println!("You guess, {guess}");
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too big")


    }
}

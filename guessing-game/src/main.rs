use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("=-=- Guessing Game -=-=");

    // Generate the number before the loop
    let secret_num: u32 = rand::thread_rng().gen_range(0..=100);

    loop {
        // Reallocate the new string each loop so that way you don't create a string frankenstein
        let mut guess = String::new();
        println!("Enter your guess: ");
        stdin().read_line(&mut guess).expect("Could not read line");
        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You're correct!");
                break;
            }
        }
    }
}

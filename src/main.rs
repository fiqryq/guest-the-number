// this is import dependency code for random number.
// check on Cargo.toml
use rand::Rng;

// this is import for standard library build it from rust.
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Simple programm using rust!");

    // random number from 1 - 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Input Your Number :");

        let mut input_number: String = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read line");

        // convert message from string to u32
        let input_number: u32 = input_number.trim().parse().expect("Please type a number");

        // compare input with random number.
        match input_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            // break the loop when number equal user input.
            Ordering::Equal => {
                println!("jackpot!!");
                break;
            }
        }
    }
}

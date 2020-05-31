use std::cmp::Ordering;
use std::io::stdin;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use regex::Regex;

fn clear_screen() {
    print!("{}[2J", 27 as char);
    // print!("\x1B[2J");
}

fn read_numeric_input() -> String {
    let numbers_re = Regex::new(r"^([0-9]*)$").unwrap();
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        // remove the trailing new line character
        input.truncate(input.len() - 1);
        if numbers_re.is_match(&input) {
            return input;
        } else {
            println!("Only numbers allowed! Try again:");
        }
    }
}

fn remember_numbers() {
    println!();
    println!("Remember the numbers!");
    println!();

    // thread-local random number generator
    let mut generator = rand::thread_rng();

    clear_screen();
    let mut sequence = String::new();
    loop {
        if sequence.is_empty() {
            println!("Please input your number (0 to 9): ");
        } else {
            println!("Please input the sequence so far, plus your new number (0 to 9): ");
        }
        let player_input = read_numeric_input();
        let input_ok = if sequence.is_empty() {
            true
        } else {
            player_input.starts_with(&sequence)
        };
        if input_ok {
            if player_input.len() > sequence.len() {
                sequence = player_input;
                let computer_num = generator.gen_range(0, 10);
                println!("Computer number: {}", computer_num);
                // add to sequence
                sequence.push_str(&computer_num.to_string());
                sleep(Duration::from_millis(1000));
                clear_screen();
            } else {
                println!("You need to add a new number to the sequence!");
            }
        } else {
            println!("You lose!");
            println!("The sequence was: '{}'. You remembered {} numbers correctly!",
                     sequence, sequence.len() - 1);
            break;
        }
    }
}

fn guess_the_number() {
    println!();
    println!("Guess the number!");
    println!();

    // thread-local random number generator
    let mut generator = rand::thread_rng();

    // the gen_range method is defined by the Rng trait that we brought into scope with the
    // use rand::Rng statement
    let secret_number = generator.gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    let mut try_number: u32 = 0;
    loop {
        try_number += 1;
        println!("Please input your guess (try #{}): ", try_number);
        let mut guess = String::new(); // call "associated function", aka static method
        stdin().read_line(&mut guess). // returns io::Result
            expect("Failed to read line");

        // This will crash the program if user enters a non-number
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Rust allows us to shadow the previous value of 'guess' with a new one.
        // This feature is often used in situations in which you want to convert a value from one
        // type to another type.
        // Because parse() can parse a variety of number types, we need to
        // tell Rust the exact number type we want by using let guess: u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("YOU WASTED A TRY, DUMMY!");
                continue;
            }
        };

        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won in {} tries!", try_number);
                break;
            }
        }
    }
}

fn main() {
    loop {
        println!();
        println!("Select which game you want to play!");
        println!();
        println!("1. Guess the number");
        println!("2. Remember the numbers");
        println!("3. Quit program");
        println!();
        println!("Enter your choice:");
        let choice: u32 = read_numeric_input().trim().parse().expect("Please type a number!");
        match choice {
            1 => guess_the_number(),
            2 => remember_numbers(),
            3 => exit(0),
            _ => println!("Error. Please enter only 1, 2 or 3 !!!"),
        }
    }
}

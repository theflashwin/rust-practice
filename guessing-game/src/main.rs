use colored::*;
use rand::Rng;
/**
 * Guessing Game
*/
use std::io;

fn main() {
    // Constants
    let DELIMITTER = "=======".to_string();
    let MIN_RANGE: i32 = 0; // inclusive
    let MAX_RANGE: i32 = 3; //inclusive
    let exit_keyword: &str = "exit";

    // Error Messages
    let INVALID_INPUT_ERR_MSG = format!(
        "[Game Error] Invalid Input - must be an integer from {} to {}",
        MIN_RANGE, MAX_RANGE
    );

    // Random Number Generator
    let mut rng = rand::thread_rng();

    // Game Fields
    let mut score: i32 = 0;
    let mut num_correct: u32 = 0;
    let mut total_attemped: u32 = 0;

    // Game Start page
    println!("{}", "Welcome to Ashwin's Guessing Game".magenta().bold());

    loop {
        let mut raw_str_guess = String::new();

        // generate new randInt
        let target = rng.gen_range(MIN_RANGE..=MAX_RANGE);

        // start new round

        println!(
            "{} Round {} {}",
            DELIMITTER.green(),
            total_attemped + 1,
            DELIMITTER.green()
        );
        println!("Enter your guess (from {} to {}): ", MIN_RANGE, MAX_RANGE);
        println!("To exit, type 'exit'");

        // read input
        if io::stdin().read_line(&mut raw_str_guess).is_err() {
            println!("{}", INVALID_INPUT_ERR_MSG.red());
            continue;
        }

        if raw_str_guess.trim() == exit_keyword.to_string() {

            if total_attemped == 0 {
                println!("{}", "See you next time!".magenta());
                return;
            }

            println!("{}. Final Score: {} | Percent Correct: {:.2}%", "Exiting Game".red(), score, (num_correct as f64 / total_attemped as f64) * 100.0);
            return;
        }

        let guess: i32 = match raw_str_guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("{}", INVALID_INPUT_ERR_MSG.red());
                continue;
            }
        };

        // bounds check
        if guess < MIN_RANGE || guess > MAX_RANGE {
            println!("{}", INVALID_INPUT_ERR_MSG.red());
            continue;
        }

        // check win
        if guess == target {
            println!("{}", "Correct!".green());
            score += 10;
            num_correct += 1;
        } else {
            println!("{} Correct guess was: {}", "Incorrect...".red(), target);
        }

        total_attemped += 1;

        println!(
            "{} Score: {} {}",
            DELIMITTER.green(),
            score,
            DELIMITTER.green()
        );
    }
}

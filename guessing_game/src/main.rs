use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    /*
        This code generates a random number between 1 and 100, stores it in the variable secret_no, and prompts the user to input their guess. The user's input is read into the variable guess and parsed as a u32. The program then compares the user's guess to the secret number using the cmp() method, and prints out an appropriate message depending on whether the user's guess was too small, too big, or equal to the secret number.

        1. rand::thread_rng() returns the random number generator that we're going to use: one that is local to the current thread of execution and seeded by the operating system.
        2. gen_range() method is defined on the Rng trait that is implemented for the type of random number generator that thread_rng returns. This method is defined to generate a random number in the range we specify.
        3. The 1..101 syntax is a half-open range that means, "Start at one, and continue up to, but not including, 101." We use a half-open range because the upper bound is exclusive.
        4. The cmp() method is defined on the Ordering trait that is implemented for the type of random number generator that thread_rng returns. This method is defined to compare two numbers, and return a negative number if the first number is less than the second, a positive number if the first number is greater than the second, and zero if they are equal.
        5. The match expression is made up of arms. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm's pattern. In this case, guess.cmp(&secret_no) is the value given to the beginning of the match expression. The match expression will go through each arm in turn, and if the value given to the beginning of the match expression fits the pattern for that arm, the code associated with that arm will be executed. If the value doesn't fit any pattern, the program will crash with an error.
        6. The Ordering type is an enum that can have the values Less, Greater, or Equal. These three values are the possible outcomes when you compare two values.
        7. The _ pattern will match any value. Since we've already handled the Ordering::Less and Ordering::Greater cases, the _ case will match only the Ordering::Equal case. The => operator separates the pattern and the code to run.
        8. The println! macro is one of the most important macros in Rust. It's used to print text to the screen. The ! indicates that this is a macro rather than a normal function. The {}s are placeholders that are replaced with the arguments that are passed to println!.
        9. The break expression in the match arm for Ordering::Equal will cause the program to exit the loop so the game will end.
        10. The expect method on Result is used to handle errors. If this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If this instance is an Ok value, expect will take the return value that Ok is holding and return it to you so you can use it in your program. In this case, the return value of expect is the number of bytes in the string that the user entered into standard input.
    */

    loop {
        println!("input your guess number.");

        let secret_no = rand::thread_rng().gen_range(1..101);

        println!("The secret number is: {}", secret_no);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_no) {
            Ordering::Less => println!("{}", "Too small!".red().bold().underline()),
            Ordering::Greater => println!("{}", "Too big!".red().bold().underline()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold().underline());
                break;
            }
        }
    }
}

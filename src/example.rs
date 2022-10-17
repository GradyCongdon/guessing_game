// use std::io;

// fn main() {
//     println!("guess");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("failed to read line");
//     println!("you guessed: {guess}");
// }

// use std::io;
// fn main() {
//   println!("guess:");
//   let mut guess = String::new();
//   io::stdin().read_line(&mut guess).expect("cannot readline");
//   println!("You guessed {guess}");
// }

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // gen_range uses rand::Rng trait, which must be scoped to use the method
    // `cargo doc --open` builds local documentation for all deps (opens in browser)
    // thread_rng is local to current thread and seeded by OS
    //  1..=100 - range expression, start..=end inclusive on both bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("the secret number is {secret_number}");
    loop {
        println!("guess");

        // Abc::xyz - associated function on Abc
        // String: expandable UTF-8
        let mut guess = String::new();

        // & - reference, immutable by default
        // .expect -> Result Ok(val) | Error(msg), program still crashes if Err encountered
        io::stdin().read_line(&mut guess).expect("cant read");
        // conversion from stdin string to number
        // specifically annotated as u32, which propgates inference to secret_number as well
        // was using expect since parse may fail
        // match expression on parse's Result
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ catchall value
            Err(_) => continue,
        };
        println!("you guessed {guess}");

        // match expression uses "arms" to compare with Ordering variant
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("...noice");
                break;
            } 
        }
    }
}

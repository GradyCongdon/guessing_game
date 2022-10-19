use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    let secret_guess = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("cannot readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("bad number");
                continue;
            }
        };

        match guess.cmp(&secret_guess) {
            Ordering::Greater => println!("too high"),
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("noice!");
                break;
            }
        };
    }
}
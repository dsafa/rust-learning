extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess a number");

  let num = rand::thread_rng().gen_range(1, 101);
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Faied to read input");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&num) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("Correct");
        break;
      }
    }
  }
}
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
   println!("Guess the number!");


   // thread_rng is a RNG that is local to the current thread seeded by the OS
   // gen_range defined by Rng trait with range stat..=end inclusive
   let secret_number = rand::thread_rng().gen_range(1..=100);

   println!("The secret number is: {secret_number}");

   println!("Please input your guess.");

   
   // mut for mutable
   // String is a string type from standard library that is growable
   // ::new() is an associated function: a function that's implemented on a type
   //         creates a new empty string
   let mut guess = String::new();

   // equivalent to std::io::stdin if we didn't have use std::io at the top
   // stdin function returns an instance of std::io::Stdin
   io::stdin()
	// we pass &mut guess as argument to read_line to tell what string to store user input in
	// recall that & is a reference, immutable by default so we do &mut guess
	.read_line(&mut guess)
   	// read_line returns a Result enum (Ok and Err)
	// an instance of Result has an expect method that you can call
   	// if it is an Err, expect will crash the program and display the message
	// If the instance of Result is Ok, expect will take the return value that Ok
	// is holding and return just that value

	// if you don't call expect, the program will compile with a warning
	.expect("Failed to read line");

   // {} works like Python almost except you don't need to specify that it's an f-string
   // e.g. can also do printlin!("x = {x} and y + 2 = {}", y + 2)
   println!("You guessed: {guess}");


   // Ordering type is an enum
   // cmp method compares two values and can be called on anything that can be compared
   // will return one of the Ordering enum types and we match on that accordingly
   match guess.cmp(&secret_number) {
   	 Ordering::Less => println!("Too small!"),
	 Ordering::Greater => println!("Too big!"),
	 Ordering::Equal => println!("You win!"),
   }

}

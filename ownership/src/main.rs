fn main() {
   // ownership is the set of rules that govern how Rust programs manage memory
   // rules:
   // - each value in Rust has an owner
   // - there can only be ONE owner at a time
   // - when the owner goes out of scope, the value will be dropped

   // the String type's data is allocated on the heap vs. string literals
   // can use something like let s = String::from("hello"); to convert
   // this knd of string can be mutated, vs. string literals because it's on the heap

   // once s is out of scope, it is no longer valid and Rust will call a function
   // "drop" that will return the memory. Like RAII in C++

   // move:
   let s1 = String::from("hello");
   let s2 = s1;
   // won't work
   // println!("{s1} world");
   // will work: 
   println!("{s2} world");
   // because by assigning s2 to s1, we do a move and s1 is considered no longer valid
   // we do this to avoid double frees
   // if you want, you can do s1.clone()

   // inverse is true:
   let mut s = String::from("hello");
   println!("{s} and goodbye");
   s = String::from("ahoy");
   // at this point, nothing refers to the original "hello" on the heap at all
   // thus, we drop it since it's out of scope
   println!("{s} world");

   // copy (stack only data):
   let x = 5;
   let y = x;
   println!("x = {x}, y = {y}");
   // types that have a known size at compile time are stored entirely on the stack
   // copies are quick to make so we have a shallow copy
   // Rust has an annotation 'Copy' trait that we can place on types stored on the stack
   // if a type implements Copy, variables do not move but are copied and Drop cannot be impled

   // passing a variable to a function will move or copy, just like assignment
   take_ownership(s);
   // s is no longer valid here since at the end of the function call, it went out of scope

   makes_copy(x);
   // since i32 implement 'Copy', a copy is made and we can use x afterward

   // the return value from give_ownership() is moved into s3
   let s3 = gives_ownership();
   println!("{s3}");

   let s4 = String::from("hello");
   // s4 is moved into takes_and_gives_back, which moves its return value into s5
   let s5 = takes_and_gives_back(s4);
   // at this point trying to do println!("{s4}"); won't work since we moved it into s5
   println!("{s5}");

   // if we wanted to let a function use a value but not take ownership, we'd have return/move it:
   let (s6, len) = calculate_length(s5);
   // also now we can't use s5 anymore since we moved it into s6!
   println!("The length of {s6} is {len}.");
}

fn take_ownership(some_string: String) {
   println!("{some_string}");
} // some_string goes out of scope and 'drop' is called -> memory freed

fn makes_copy(some_integer: i32) {
   println!("{some_integer}");
} // since a copy was made nothing happens when some_integer goes out of scope here

fn gives_ownership() -> String {
   let some_string = String::from("yours");

   some_string
}

fn takes_and_gives_back(a_string: String) -> String {
   // a_string comes into scope

   // a_string is returned and moves out to the calling function
   a_string
}

fn calculate_length(s: String) -> (String, usize) {
   let length = s.len();

   // we can return multiple values but tupling them and then use them by unpacking
   (s, length)
}
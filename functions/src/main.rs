fn main() {
   print_labeled_measurement(5, 'h');

   let x = plus_one(five());
   println!("The value of x called from plus_one(five()) is {x}");
}

// function signatures in rust must declare the type of each argument
fn print_labeled_measurement(value: i32, unit_label: char) {
   println!("The measurement is {value}{unit_label}");
}

// statements are instructions that perform some action and do not return a value (i.e. let y = 6)
// expressions evaluate to a resultant value (i.e. 5 + 6)
// expressions do not include ending semicolons, if they do it's a statement
// i.e. this expression (also, scope block creation with curly braces is an expression)
// {
//	let x = 3;
//	x + 1
// }
// most functions return the last expression implicitly
fn five() -> i32 {
   5
}

fn plus_one(x: i32) -> i32 {
   // adding a semicolon here will produce an error since it will now return (), the unit type
   x + 1
}
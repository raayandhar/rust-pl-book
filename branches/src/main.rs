fn main() {
   // if, else if, else all work about how you expect
   // since if is an expression, we can use it on RHS of statements
   // types must be matched though!
   let condition = true;
   let number = if condition { 5 } else { 6 };

   println!("The value of number is {number}");

   // loops
   // loop { } is like a while loop, and you can use break; and continue; for flow
   // to return something out of a loop, start by let res = loop { ... break val; }
   // you can also return from inside a loop; return always exits the current function
   // loop label i.e. 'labeled loop specifies within nested loops how to break i.e. break 'labeled
   // while loops work about as you expect:
   // while cond { statements and expressions }
   // for loops can be somewhat pythonic, i.e. for an array a:
   // for element in a { ... }
   // you can also use a Range and the methods on them, i.e.
   // for number in (1..4).rev() { ... }
}

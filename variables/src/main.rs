fn main() {
   let mut x = 5;
   
   // we create a new variable x
   // when we use the let keyword again, we're creating a new variable (vs. defining it as mut)
   // this allows us to let the same "variabl" (name) have different types
   let x = x + 1;
   println!("The value of x is {x}");

   {
	// in this scope, this x *overshadows* the second, i.e. we shadowed x
	// but this is a new variable as well
	let x = x * 2;
	println!("The value of x in the inner scope is: {x}");
   }

   println!("The value of x is {x}");

   // constants are always immutable, must always be annotated,
   // and valid for the entire time a program runs in their declared scope
   // const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

   // data types
   // Rust is statically typed; we must know types of all variables at compile time
   // ints (various types), floats (default f64), bool, char (''), string ("")
   // compound types group multiple values into one type:
   // tuples i.e. let tup: (i32, f64, u8) = (500, 6.4, 1);
   // use pattern matching to deconstruct, i.e. let (x, y, z) = tup; or tup.0, tup.1, etc.
   // arrays, must have same type i.e. let a: [i32; 5] = [1, 2, 3, 4, 5] or let a = [3; 5];
   // array is fixed size because it's on the stack, vectors are on the heap
   // index into like as you expect i.e. a[i];
}


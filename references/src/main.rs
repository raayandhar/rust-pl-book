fn main() {
   // a reference is like a pointer in that it's an address we can follow to access
   // the data stored at the address; that data is owned by some other variable
   // unlike a pointer, a reference is guaranteed to point to a valid value of
   // a particular type for the life of that reference

   let s1 = String::from("hello");

   // &s1 creates a reference that *refers* to the value of s1 but does not own it
   // we call the action of creating a reference -borrowing-
   let len = calculate_length(&s1);

   println!("The length of '{s1}' is {len}.");

   // we can create a reference that can be modified via mutable reference
   let mut s = String::from("hello");
   change(&mut s);
   // however, you can have no other references to a value if you already have
   // a mutable reference to it. this prevents -data races- at compile time
   // data race can occur when:
   // * two or more pointers access the same data at the same time
   // * at least one of the pointers is being used to write to the data
   // * there's no mechanism being used to synchronize access to the data
   // we can have multiple mutable references within nested scopes, i.e.
   // as long as the mutable reference goes out of scope, we can create a new one
   // you also cannot combine mutable and immutable references, as this is bad code
   // but the compiler is smart and if can naturally "scope" things itself

   // dangling references:
   // a pointer that references a location in memory that may have been given
   // to someone else, by freeing memory while preserving a pointer to that memory
   // rust compiler guarantees that this can't happen: if you have a reference
   // to some data, the compiler ensures that the data will not go out of scope
   // before the reference to the data does.
}

fn change(some_string: &mut String) {
   some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
   // we cannot modify something we're boworring. i.e.
   // some_string.push_str(", world"); will cause an error
   s.len()
} // here, s goes out of scope. since s does NOT have ownership of what
  // it refers to, the String is NOT dropped

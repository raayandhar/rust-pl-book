fn main() {
   // slices let us reference a contiguous sequence of elements in a collection
   // a slice is a kind of reference, so it does not have ownership

   let mut s = String::from("hello world");

   let word = first_word(&s); // word gets the value 5

   s.clear(); // this empties the String, making it equal to ""

   // word still has the value 5 here, but s no longer has any content we
   // could meaningfully use with the value 5, so word is now totally invalid!
   let s_len = s.len();
   println!("word {word} neq s.len() {s_len}");


   // a string slice is a reference to a contiguous sequence of the elements of a String
   let _s1 = String::from("hello world");
   // slice range defined as [starting_index..ending_index]
   // starting_index is the first pos in the slice
   // ending_index is one more than the last position in the slice
   // fancy slice tricks:
   // &s[3..len] == &s[3..] and &s[0..len] == &s[..]
   let _hello = &_s1[0..5];
   let _world = &_s1[6..11];
   // as you would expect, we can slice arrays:
   let a = [1, 2, 3, 4, 5];

   let slice = &a[1..3];

   assert_eq!(slice, &[2, 3]);
}

// suppose we wanted to write a function that takes a string of words separated
// by spaces and returns the first word it finds in that string
// we will return the index of the end of the word, indicated by a space

fn first_word(s: &String) -> usize {
   // because we need to through the String element by element
   // we can convert it to an array of bytes via as_bytes
   let bytes = s.as_bytes();

   // enumerates wrap result fo iter and returns index and reference to element
   for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
       	  return i;
       }
   }

   s.len()
}

// slice version:
// now, when we find a space, we return a string slice
// and the compiler will ensure that references into the String remain valid
// trying to call s.clear(); after if word is used after in the scope will error
// apparently the function API can also just do (s &str) -> &str via deref coercions (?)
fn first_word_slice(s: &String) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
       	  return &s[0..i];
       }
   }

   &s[..]
}


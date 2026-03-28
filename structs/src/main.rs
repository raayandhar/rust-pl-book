struct User {
    active: bool, // fields
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
   // can create an instance of a struct like this
   let mut user1 = User {
       active: true,
       username: String::from("someusername123"),
       email: String::from("someone@example.com"),
       sign_in_count: 1,
   };

   // access values with dot notation
   user1.email = String::from("anotheremail@example.com");

   // Rust will not let us only mark certain fields as mutable, has to be all

   // struct update syntax:
   let user2 = User {
       active: user1.active,
       username: user1.username,
       email: String::from("another@example.com"),
       sign_in_count: user1.sign_in_count,
   };
   // or even quicker:
   let user3 = User {
       email: String::from("another@example.com"),
       ..user2
   };
   // in each case we are doing *moves* on the fields that we are taking from the
   // other instance that require move (i.e. String), so this other instance is unusable
   // the other fields (that use Copy) are still ok

   // tuple structs:
   // names unassociated
   let black = Color(0, 0, 0);

   // unit-like structs:
   // behave similarly to ()
   let subject = AlwaysEqual;

   // to do structs with reference fields requires lifetimes which we will discuss later
}

// field init shorthand:
fn build_user(email: String, username: String) -> User {
   User {
       active: true,
       username,
       email,
       sign_in_count: 1,


   }
}

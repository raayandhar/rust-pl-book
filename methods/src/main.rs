#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// methods:
// like functions except are defined within the context of a struct/enum/trait object
// their first parameter is always self (instance of what it's being called on)

// we start with an impl block to be associated with the Rectangle type
// &self is short for self: &Self
// if we wanted to changes things we'd use &mut self
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we could have split these into multiple impl Rectangle { ... } blocks
    fn width(&self) -> bool {
        self.width > 0
    }

    // associated functions:
    // all functions defined with an impl block that don't have self
    // as a their first parameters (this not methods)
    // i.e. String::from
    // to call, we do something like Rectangle::square(3) so we
    // namespace the struct
    fn square(size: u32) -> Self {
        Self {
	    width: size,
	    height: size,
	}
    }
}


fn main() {
    let rect1 = Rectangle {
    	width: 30,
	height: 50,
    };

    println!("The area of the rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Rust does -automatic referencing-. when we call a method with
    // object.something(), Rust automatically adds in &, &mut or * so
    // that the object matches the signature of the method.
}

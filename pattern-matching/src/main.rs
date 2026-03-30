// Rust has this Option enum that is basically the same as Option in OCaml 
// with the same principle as to why we use it. It's included in the prelude
// so we don't need to bring it into scope explicitly (same for Some and None),
// also included in the prelude so you don't need to namespace / use Option::
// <T> is generic type parameter
// Some(T) and None are variants of type Option<T>
enum Option<T> {
    None,
    Some(T),
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // we have this match control flow which
    // is basically the pattern-matching idea
    // from OCaml except we don't really have
    // the recursion concept.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    // of course we have the classic Option<T> match pattern
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
    // for example
    // match arm patterns must cover all possibilities
    // we can catch by naming some variable and then using it:
    //     other => function(other),
    // where earlier we already matched the values we wanted
    // if we don't want to bind to variable, we can use catch-all:
    //     _ => (), // returning nothing, or i.e. function(),

    // if let takes a pattern and an expression separated by an
    // equal sign, and tries to bind if we match, else return ()
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        // this is the catch-all (_) portion we can implement
        println!("Failed to match.")
    }
}

// we also have let...else where we can take a pattern on
// the LHS and an expression on the RHS, but instead of an if
// branch, only an else. we try to match - if successful, we bind
// to the value, otherwise we take the else branch, which MUST return
// i.e.
fn describe_state_quarter(coin: Coin) -> std::option::Option<Coin> {
    // don't really know what's going on here but I got it to work
    // thanks to the rust compiler
    let Coin::Quarter(ref _state) = coin else {
        return None;
    };

    Some(coin)
}

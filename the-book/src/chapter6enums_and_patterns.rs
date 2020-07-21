use crate::typeutils;

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Like structs, enums can have methods too
impl IpAddr {
    fn home() -> IpAddr {
        return IpAddr::V4(127, 0, 0, 1);
    }
}

fn route(ip_addr: IpAddr) {
    println!("Routing {:?}", ip_addr);
}

pub(crate) fn enums() {
    println!();
    println!("6. Enums");
    println!();

    // Enums are a feature in many languages, but their capabilities differ in each language.
    // Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.

    let home = IpAddr::home();
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);
}

pub(crate) fn option_enum() {
    println!();
    println!("6. Option Enum");
    println!();

    // Option is an enum defined by the standard library.
    // It encodes the very common scenario in which a value could be something or it could be nothing.
    // Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the
    // cases you should be handling; this functionality can prevent bugs that are extremely common in other
    // programming languages (e.g. Java's NullPointerException).
    // Rust doesn’t have the null feature that many other languages have. However, the concept that null is trying to
    // express is still a useful one: a null is a value that is currently invalid or absent for some reason.
    //    enum Option<T> {
    //      Some(T),
    //      None,
    //    }

    // if we try to use a variable before giving it a value, we’ll get a compile-time error, which shows that Rust
    // indeed does not allow null values.
    let absent_number: i32;
    // error[E0381]: borrow of possibly-uninitialized variable: `absent_number`
    // println!("absent_number: {}", absent_number);

    // we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type that the
    // Some variant will hold by looking only at a None value.
    let absent_number: Option<i32> = None;
    println!("type_of(absent_number): {}", typeutils::type_of(&absent_number));

    // So why is having Option<T> any better than having null?
    // In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us
    // use an Option<T> value as if it were definitely a valid value.
    // When we have an Option<T> we have to worry about possibly not having a value, and the compiler will make sure
    // we handle that case before using the value.
    // In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
    // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null
    // when it actually is.

    let x: i8 = 4;
    let y: Option<i8> = Some(6);
    // error[E0277]: cannot add `std::option::Option<i8>` to `i8`
    // let sum = x + y;
    let sum = if y.is_none() { x } else { x + y.unwrap() };
    println!("Sum x + y = {}", sum);

    let sum = x + y.unwrap_or(0);
    println!("Sum (even more concise) = {}", sum);
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        // A match arm has two parts: a pattern and some code
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub(crate) fn pattern_matching() {
    println!();
    println!("6. Pattern matching");
    println!();

    // Rust has an extremely powerful control flow operator called match that allows you to compare a value against a
    // series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal
    // values, variable names, wildcards, and many other things. The power of match comes from the expressiveness
    // of the patterns and the fact that the compiler confirms that all possible cases are handled.

    let coin = Coin::Dime;
    println!("value of {:?} = {} cents", coin, value_in_cents(&coin));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

    // The _ placeholder
    // Rust also has a pattern we can use when we don’t want to list all possible values.
    let some_u8_value = 7u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern
    // while ignoring the rest.
    // Consider this code that matches on an Option<u8> value but only wants to execute code if the value is 3.
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // Instead, we could write this in a shorter way using if let.
    // You can think of if let as syntax sugar for a match that runs code when the value matches one pattern and
    // then ignores all other values.
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

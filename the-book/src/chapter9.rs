use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// Rust groups errors into two major categories: recoverable and unrecoverable errors.
// For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user
// and retry the operation.
// Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

// Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors
// and the panic! macro that stops execution when the program encounters an unrecoverable error.

pub(crate) fn error_handling_panic() {
    println!();
    println!("9. Error Handling - panic! macro");
    println!();

    // Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases,
    // Rust has the panic! macro. When the panic! macro executes, your program will print a failure message,
    // unwind and clean up the stack, and then quit. This most commonly occurs when a bug of some kind has been
    // detected and it’s not clear to the programmer how to handle the error.

    // panic!("crash and burn"); // thread 'main' panicked at ...
}

pub(crate) fn error_handling_result() {
    println!();
    println!("9. Error Handling - Result");
    println!();

    // The Result enum is defined as having two variants, Ok and Err, as follows:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // The T and E are generic type parameters:
    // T represents the type of the value that will be returned in a success case within the Ok variant, and
    // E represents the type of the error that will be returned in a failure case within the Err variant.

    let result = File::open("hello.txt");
    let f_with_match = match result {
        // Note that, like the Option enum, the Result enum and its variants have been brought into scope by the
        // prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("f_with_match: {:?}", f_with_match);

    // The match expression is very useful but also very much a primitive.
    // The Result<T, E> type has many methods that accept a closure and are implemented using match expressions.
    // Using those methods makes our code more concise:
    let f_with_closure = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("f_with_closure: {:?}", f_with_closure);

    // Result<T, E> type has many helper methods defined on it to do various tasks.
    // One of those methods, called unwrap, is a shortcut method that is implemented just like the match expression
    // we wrote above. If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us.

    // If we run this code without a hello.txt file, we’ll see an error message from the panic!
    let f_with_unwrap = File::open("hello.txt").unwrap();
    println!("f_with_unwrap: {:?}", f_with_unwrap);

    // Another method, expect, which is similar to unwrap, lets us also choose the panic! error message.
    // Using expect instead of unwrap and providing good error messages can convey your intent and make tracking
    // down the source of a panic easier.
    let f_with_expect = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("f_with_expect: {:?}", f_with_expect);
}

pub(crate) fn propagating_errors_manually() -> Result<String, io::Error> {
    println!();
    println!("9. Error Handling - Propagating Errors (manually)");
    println!();

    // When you’re writing a function whose implementation calls something that might fail, instead of handling the
    // error within this function, you can return the error to the calling code so that it can decide what to do.
    let result = File::open("hello.txt");
    let mut f = match result {
        Ok(file) => file,
        Err(e) => return Err(e), // return errors to the calling code
    };

    let mut s = String::new();
    // We don’t need to explicitly say return, because this is the last expression in the function.
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub(crate) fn propagating_errors_with_operator() -> Result<String, io::Error> {
    println!();
    println!("9. Error Handling - Propagating Errors (with ? operator)");
    println!();

    // This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ?
    // to make this easier.

    let mut s = String::new();

    // The ? placed after a Result value is defined to work in almost the same way as the match expressions we
    // defined to handle the Result values above. If the value of the Result is an Ok, the value inside the Ok will
    // get returned from this expression, and the program will continue.
    // If the value is an Err, the Err will be returned from the whole function as if we had used the return
    // keyword so the error value gets propagated to the calling code.
    let mut f = File::open("hello.txt")?;
    f.read_to_string(&mut s)?;

    // We could even shorten this code further by chaining method calls immediately after the ?, as shown below:
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Reading a file into a string is a fairly common operation, so Rust provides the convenient fs::read_to_string
// function that opens the file, creates a new String, reads the contents of the file, puts the contents into
// that String, and returns it.
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
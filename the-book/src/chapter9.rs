use std::fs::File;
use std::io::ErrorKind;

// Rust groups errors into two major categories: recoverable and unrecoverable errors.
// For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user
// and retry the operation.
// Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

// Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors
// and the panic! macro that stops execution when the program encounters an unrecoverable error.

pub(crate) fn error_handling_panic() {
    println!();
    println!("9. Error Handling - panic!");
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
}

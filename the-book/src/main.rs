// Based on online book: The Rust Programming Language
// https://doc.rust-lang.org/book/title-page.html

use futures::executor::block_on;

mod typeutils;
mod chapter3commonconcepts;
mod chapter4ownership;
mod chapter5structs_and_methods;
mod chapter6enums_and_patterns;
mod chapter8collections;
mod chapter9error_handling;
mod chapter10generics;
mod chapter11unit_tests;
mod chapter15concurrency;
mod chapter19macros;
// the above lines tell Rust to load the contents of the modules from files with the same name as the module

async fn print_one() {
    print!(" 1 ");
}

async fn print_one_two() {
    // Inside an async fn, you can use .await to wait for the completion of another type that implements
    // the Future trait, such as the output of another async fn.
    // Unlike block_on(), .await doesn't block the current thread, but instead asynchronously waits for
    // the future to complete.
    print_one().await;
    print!(" 2 ");
}

async fn print_three() {
    print!(" 3 ");
}

async fn print_one_two_three_maybe() {
    let f1 = print_one_two();
    let f2 = print_three();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in one future, another
    // future will take over the current thread. If both futures are blocked, then
    // this function is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    println!("async/await example");
    println!();

    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread.
    block_on(print_one_two_three_maybe());

    println!();

    chapter3commonconcepts::variables();
    chapter3commonconcepts::scalar_data_types();
    chapter3commonconcepts::compound_data_types_tuples_and_arrays();
    chapter3commonconcepts::control_flow();

    chapter4ownership::move_with_variables();
    chapter4ownership::move_with_functions();
    chapter4ownership::borrow();

    chapter5structs_and_methods::plain_structs();
    chapter5structs_and_methods::tuple_structs();
    chapter5structs_and_methods::struct_newtypes();
    chapter5structs_and_methods::methods();

    chapter6enums_and_patterns::enums();
    chapter6enums_and_patterns::option_enum();
    chapter6enums_and_patterns::pattern_matching();

    chapter8collections::collections_vectors();

    chapter9error_handling::panic();
    chapter9error_handling::result();
    let result = chapter9error_handling::propagating_errors_manually();
    println!("result {:?}", result);
    let result = chapter9error_handling::propagating_errors_with_operator();
    println!("result {:?}", result);

    chapter10generics::generic_types();
    chapter10generics::traits();
    chapter10generics::lifetimes();

    chapter15concurrency::threads();

    chapter19macros::declarative_macros();
    chapter19macros::procedural_macros();
}


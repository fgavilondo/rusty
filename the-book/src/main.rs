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
// the above lines tell Rust to load the contents of the modules from files with the same name as the module

// Based on online book: The Rust Programming Language
// https://doc.rust-lang.org/book/title-page.html

fn main() {
    println!();

    chapter3commonconcepts::variables();
    chapter3commonconcepts::scalar_data_types();
    chapter3commonconcepts::compound_data_types();
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
}


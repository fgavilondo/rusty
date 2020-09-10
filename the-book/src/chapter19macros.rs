// Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
// The term macro refers to a family of features in Rust:
//
// 1) Declarative Macros with macro_rules!
//
// Declarative Macros match against patterns and replace the code with other code.
// e.g. println! and vec!
//
// 2) Procedural Macros
//
// Procedural macros accept some code as an input, operate on that code, and produce some code as an output:
//
// 2.1) Custom #[derive] macros specify code added with the derive attribute used on structs and enums
// 2.2) Attribute-like macros
//  Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute,
//  they allow you to create new attributes. They’re also more flexible: derive only works for structs and enums;
//  attributes can be applied to other items as well, such as functions.
// 2.3) Function-like macros
//  Function-like macros define macros that look like function calls but operate on the tokens specified as
//  their argument.
//  Similarly to macro_rules! macros, they’re more flexible than functions; for example, they can take an unknown
//  number of arguments.


// The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which the
// macro is defined is brought into scope.
#[macro_export]
macro_rules! myvec {
    // The structure of the macro body is similar to the structure of a match expression.
    // Here we have one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated
    // with this pattern.
    // If the pattern matches, the associated block of code will be emitted.  Given that this is the only pattern
    // in this macro, there is only one valid way to match; any other pattern will result in an error.
    // More complex macros will have more than one arm.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub(crate) fn declarative_macros() {
    println!();
    println!("19.5 Macros - Declarative macros");
    println!();

    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // it’s more common to create a Vec<T> that has initial values, and Rust provides the vec! macro for convenience.
    let mut vec_v = vec![1, 2, 3];
    println!("v = {:?}", vec_v);

    let mut simplevec_v = myvec![4, 5, 6];
    println!("v = {:?}", simplevec_v);
}

pub(crate) fn procedural_macros() {
    println!();
    println!("19.5 Macros - Procedural macros");
    println!();
}


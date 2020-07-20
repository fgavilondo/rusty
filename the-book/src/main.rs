mod typeutils;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;
mod chapter8;
mod chapter9;
mod chapter10;
// the above lines tell Rust to load the contents of the modules from files with the same name as the module

// Based on online book: The Rust Programming Language
// https://doc.rust-lang.org/book/title-page.html

fn main() {
    println!();

    // chapter3::common_concepts_variables();
    // chapter3::common_concepts_scalar_data_types();
    // chapter3::common_concepts_compound_data_types();
    // chapter3::common_concepts_control_flow();
    //
    // chapter4::ownership_move_with_variables();
    // chapter4::ownership_move_with_functions();
    // chapter4::ownership_borrow();
    //
    // chapter5::structs();
    // chapter5::tuple_structs();
    // chapter5::struct_newtypes();
    // chapter5::methods();
    //
    // chapter6::enums();
    // chapter6::option_enum();
    // chapter6::pattern_matching();
    //
    // chapter8::collections_vectors();
    //
    // chapter9::error_handling_panic();
    // chapter9::error_handling_result();
    // let result = chapter9::propagating_errors_manually();
    // println!("result {:?}", result);
    // let result = chapter9::propagating_errors_with_operator();
    // println!("result {:?}", result);

    chapter10::generic_types();
}


pub(crate) fn move_with_variables() {
    println!();
    println!("4. Ownership - Move - Variables");
    println!();

    // For primitive (scalar) types, an assignment is a copy, not a move,
    // as these are stack values, there is no heap memory to keep track of.
    // Stack values implement the Copy trait.

    let i = 42;
    let mut i_copy = i;
    i_copy += 1;
    println!("i = {}", i);
    println!("i_copy = {}", i_copy); // 43

    // However, for heap variables an assignment transfers ownership:

    let s = String::from("hello"); // s comes into scope
    let s2 = s; // s2 takes ownership of the value "hello"

    // Move is like a "shallow copy", meaning we copy the pointer, the length, and the capacity
    // that are on the stack. We do not copy the data on the heap that the pointer refers to.
    // But once you move ownership, you cannot use the old variable anymore:
    // println!("s = {}", s); // compile error: "value borrowed here after move"

    // allocated heap memory will be freed when s2 goes out of scope (s is ignored from now on)
    println!("s2 = {}", s2);

    // use clone() for "deep copy"
    let s2_clone = s2.clone() + " world!";
    println!("s2clone = {}", s2_clone);

    // you can still use the cloned variable as the clone is a separate copy, not a move
    println!("s2 still = {}", s2);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("some_string = {}", some_string);
    // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("some_integer = {}", some_integer);
    // Here, some_integer goes out of scope. Nothing special happens.
}

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let s = String::from("i came from a function"); // s comes into scope
    s
    // s is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string
    // a_string is returned and moves out to the calling function
}

fn calculate_length_moved(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

pub(crate) fn move_with_functions() {
    println!();
    println!("4. Ownership - Move - Functions");
    println!();

    // The semantics for passing a value to a function are similar to those for assigning a value
    // to a variable. Passing a variable to a function will move (heap types) or copy (scalar types),
    // just as assignment does.

    let s = String::from("hello"); // s comes into scope

    // s's value moves into the function and so is no longer valid here
    takes_ownership(s);
    // compile error: "borrow of moved value: s"
    // println!("s = {}", s);

    let x = 5;  // x comes into scope
    makes_copy(x);
    // x would move into the function, but i32 implements Copy trait (stack value),
    // so it’s okay to still use x afterward
    println!("x = {}", x);

    // Returning values from functions can also transfer ownership:

    let s_that_came_from_a_function = gives_ownership(); // gives_ownership moves its return value into s2
    println!("s_that_came_from_a_function = {}", s_that_came_from_a_function);

    let s3 = String::from("hello");    // s3 comes into scope
    let s_passed_through_a_function = takes_and_gives_back(s3);
    // s3 is moved into takes_and_gives_back(), which also moves its return value into s4
    println!("s_passed_through_a_function = {}", s_passed_through_a_function);
    // println!("s3 = {}", s3); // value borrowed here after move

    // Taking ownership and then returning ownership with every function is a bit tedious.
    // What if we want to let a function use a value but not take ownership?
    // It’s quite annoying that anything we pass in also needs to be passed back if we want to
    // use it again, in addition to any data resulting from the body of the function that we
    // might want to return as well.
    // It’s possible to return multiple values using a tuple:

    let (s5, len) = calculate_length_moved(s_passed_through_a_function);
    println!("The length of '{}' is {}.", s5, len);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
    // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.
}

fn try_to_change_borrowed_immutable(some_string: &String) {
    // Just as variables are immutable by default, so are references.
    // We’re not allowed to modify something we have a reference to.
    // Compile error: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // some_string.push_str(", world");
    println!("some_string in change_borrowed() = {}", some_string);
}

fn change_borrowed_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

pub(crate) fn borrow() {
    println!();
    println!("4. Ownership - References and Borrowing");
    println!();

    // The Rules of References:
    // * At any given time, you can have either one mutable reference or any number of immutable references.
    // * References must always be valid.

    // The issue with the calculate_length_moved() function is that we have to return the String to
    // the calling function so we can still use the String after the call to calculate_length,
    // because the String was moved into calculate_length.
    // Here is how you would define and use a calculate_length function that has a reference to an
    // object as a parameter instead of taking ownership of the value:
    let hello_str = String::from("hello");
    // These ampersands are references, and they allow you to refer to some value without taking
    // ownership of it.
    let len = calculate_length(&hello_str);
    println!("The length of '{}' is {}.", hello_str, len);

    // Note: The opposite of referencing by using & is dereferencing, which is accomplished with
    // the dereference operator, * (Chapter 15)

    // We call having references as function parameters 'borrowing'.
    // So what happens if we try to modify something we’re borrowing? It doesn’t work!
    try_to_change_borrowed_immutable(&hello_str);

    // We can fix the error with just a small tweak:
    let mut hello_mut = String::from("hello");
    change_borrowed_mutable(&mut hello_mut);
    // First, we had to change s to be mut. Then we had to create a mutable reference with &mut s
    // and accept a mutable reference with some_string: &mut String.

    // But mutable references have one big restriction: you can have only one mutable reference to
    // a particular piece of data in a particular scope. This code will fail:
    let mut s_mut = String::from("hello");
    let r1 = &mut s_mut;
    // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("r1 = {}", r1);

    // The benefit of having this restriction is that Rust can prevent data races at compile time:
    // * Two or more pointers access the same data at the same time.
    // * At least one of the pointers is being used to write to the data.
    // * There’s no mechanism being used to synchronize access to the data.

    // We can use curly brackets to create a new scope, allowing for multiple mutable references,
    // just not simultaneous ones:
    let mut s2_mut = String::from("hello");
    {
        let r1 = &mut s2_mut;
        println!("r1 = {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s2_mut;
    println!("r2 = {}", r2);

    // warning: variable does not need to be mutable
    let mut s3 = String::from("hello");
    let r3 = &s3; // no problem
    let r4 = &s3; // no problem

    // We also cannot have a mutable reference while we have an immutable one:
    // Compile error[E0502]: cannot borrow `s3` as mutable because it is also borrowed as immutable
    // let r5 = &mut s3; // BIG PROBLEM

    println!("{}, {}", r3, r4);

    // any borrow must last for less than the scope of the owner
}

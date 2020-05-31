// Based on online book: The Rust Programming Language
// https://doc.rust-lang.org/book/title-page.html

// You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated:
const CONST_FIVE: u32 = 5;

fn ch3_common_concepts_variables() {
    println!();
    println!("3.1. Variables and Mutability");
    println!();

    // Variables are immutable by default
    let x = 5;
    // x = 6; // error
    println!("x = {}", x);

    // Mutability is opt-in:
    let mut mutx = 5;
    println!("mutx = {}", mutx);
    mutx = 6;
    println!("mutx = {}", mutx);

    // FIVE = 6; // runtime error: cannot assign to this expression
    println!("Constant = {}", CONST_FIVE);

    // Shadowing: You can declare a new variable with the same name as a previous variable,
    // and the new variable "shadows" the previous variable.
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x); // 12

    // By using let, we can perform a few transformations on a value but have the variable be
    // immutable after those transformations have been completed.
    // Because we’re effectively creating a new variable when we use the let keyword again,
    // we can change the type of the value but reuse the same name:
    let dashes = "---";
    println!("dashes as &str: {}", dashes);
    let dashes = dashes.len();
    println!("shadowed dashes:  {}", dashes);

    // note: String not the same as &str
    let mut dashes: String = String::from("---");
    println!("dashes as String: {}", dashes);
    dashes.push('=');
    println!("dashes modified: {}", dashes);
    let a_slice = &dashes[3..];
    println!("slice 3..len: {}", a_slice);
    dashes.clear();
    println!("dashes cleared: {}", dashes);

    // str is an immutable1 sequence of UTF-8 bytes of dynamic length somewhere in memory.
    // Since the size is unknown, one can only handle it behind a pointer.
    // This means that str most commonly2 appears as &str: a reference to some UTF-8 data,
    // normally called a "string slice" or just a "slice".

    // String is the dynamic heap string type, like Vec: use it when you need to own or modify
    // your string data.

    // String keeps the buffer and is very practical to use. &str is lightweight and should be used
    // to "look" into strings. You can search, split, parse, and even replace chunks without
    // needing to allocate new memory.
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn ch3_common_concepts_scalar_data_types() {
    println!();
    println!("3.2.1 Scalar Data Types");
    println!();

    println!("Integers:");
    let i = 1000;
    println!("i = {}", i);
    println!("Type of i = {}", type_of(&i));

    // let int_overflow: u8 = 300; // compile error

    println!("Floating Point:");
    let f = 2.0;
    println!("f = {}", f);
    println!("Type of f = {}", type_of(&f));

    println!("Booleans:");
    let b = true;
    println!("b = {}", b);
    println!("Type of b = {}", type_of(&b));

    println!("Characters:");
    let c = '😻';
    println!("c = {}", c);
    println!("Type of c = {}", type_of(&c));
}

fn plus_one(arr: &mut [i32; 5]) {
    for i in 0..arr.len() {
        arr[i] = arr[i] + 1;
    }
}

fn ch3_common_concepts_compound_data_types() {
    println!();
    println!("3.2.2 Compound Data Types");
    println!();

    println!("Tuples:");
    // A tuple is a general way of grouping together a number of values with a variety of types
    // into one compound type. Tuples have a fixed length:
    // once declared, they cannot grow or shrink in size.
    let tup1 = (500, 6.4, 'z');
    println!("tup1 = {:?}", tup1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure it:
    let (i, f, c) = tup1;
    println!("i = {}, f = {}, c = {}", i, f, c);
    // In addition to destructuring through pattern matching, we can access a tuple element directly
    // by using a period (.) followed by the index of the value we want to access:
    let tup1_1 = tup1.1;
    println!("tup1_1 = {}", tup1_1);

    let tup2: (i32, f64, u8) = (500, 6.4, 1); // with explicit type annotation
    println!("tup2 (pretty print) = {:#?}", tup2);

    println!();
    println!("Arrays:");
    // Arrays are useful when you want your data allocated on the stack rather than the heap,
    // or when you want to ensure you always have a fixed number of elements.
    // As opposed to Tuples, all values in an Array must have the same type.
    let arr1 = [1, 2, 3, 4, 5];
    println!("arr1 = {:?}", arr1);
    println!("Fist elem: {}", arr1[0]);
    println!("Last elem: {}", arr1[4]);
    // Runtime exception 'index out of bounds: the len is 5 but the index is 10'
    // println!("{}", arr[5]);

    // Create an array that contains the same value for each element:
    let mut arr2 = [3; 5]; // same as let a = [3, 3, 3, 3, 3];
    println!("[3; 5] = {:?}", arr2);

    // iterating over arrays and modifying elements

    plus_one(&mut arr2);
    println!("After plus_one() = {:?}", arr2); //  [4, 4, 4, 4, 4];

    println!();
    println!("Vector (Note: not a Compound Data Type!)");

    // Note: vectors are the heap equivalent of arrays, they can grow or shrink in size:
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    v.push(6);
    println!("v = {:?}", v);
}

fn ch3_common_concepts_control_flow() {
    println!();
    println!("3.4 Control Flow");
    println!();

    println!("if expressions:");
    let number = 12;
    if number % 4 == 0 { // the good old modulo division trick
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

    // Because if is an expression, we can use it on the right side of a let statement:
    let condition = true;
    let i = if condition {
        5
    } else {
        6
    };
    println!("condition = {}, i = {}", condition, i);

    println!();
    println!("loops");
    // Loops are expressions too:
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // return something from the loop
        }
    };
    println!("The loop result is {}", result); // 20

    println!();
    println!("conditional loops:");

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    println!();
    println!("looping through a collection with 'for'");
    let mut arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("the value is: {}", element);
    }

    for i in 0..arr.len() {
        arr[i] = arr[i] + 1;
    }

    for number in (1..4).rev() { // range
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}

fn ch4_ownership_move_with_variables() {
    println!();
    println!("4. Ownership - Move - Variables");
    println!();

    let s = String::from("hello"); // s comes into scope
    let s2 = s; // s2 takes ownership of the value "hello"
    // allocated heap memory will be freed when s2 goes out of scope (s is ignored from now on)
    println!("s2 = {}", s2);
    // Move is like a "shallow copy", but once you move ownership,
    // you cannot use the old variable anymore:
    // println!("s = {}", s); // compile error: "value borrowed here after move"

    // use clone() for "deep copy"
    let s2_clone = s2.clone();
    println!("s2clone = {}", s2_clone);
    // you can still use the cloned variable as the clone is a separate copy, not a move
    println!("s2 = {}", s2);

    // note hat for primitive (scalar) types, an assignment is a copy, not a move,
    // as these are stack values, there is no heap memory to keep track of

    let i = 42;
    let i2 = i;
    println!("i = {}", i);
    println!("i_copy = {}", i2);
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

fn ch4_ownership_move_with_functions() {
    println!();
    println!("4. Ownership - Move - Functions");
    println!();

    // The semantics for passing a value to a function are similar to those for assigning a value
    // to a variable. Passing a variable to a function will move or copy, just as assignment does.

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

    let s2 = gives_ownership(); // gives_ownership moves its return value into s2
    println!("s2 = {}", s2);

    let s3 = String::from("hello");    // s3 comes into scope
    let s4 = takes_and_gives_back(s3);
    // s3 is moved into takes_and_gives_back(), which also moves its return value into s4
    println!("s4 = {}", s4);
    // println!("s3 = {}", s3); // value borrowed here after move

    // Taking ownership and then returning ownership with every function is a bit tedious.
    // What if we want to let a function use a value but not take ownership?
    // It’s quite annoying that anything we pass in also needs to be passed back if we want to
    // use it again, in addition to any data resulting from the body of the function that we
    // might want to return as well.
    // It’s possible to return multiple values using a tuple:

    let (s5, len) = calculate_length_moved(s4);
    println!("The length of '{}' is {}.", s5, len);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
    // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.
}

fn change_borrowed(some_string: &String) {
    // Just as variables are immutable by default, so are references.
    // We’re not allowed to modify something we have a reference to.
    // Compile error: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // some_string.push_str(", world");
    println!("some_string in change_borrowed() = {}", some_string);
}

fn change_borrowed_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

fn ch4_ownership_borrow() {
    println!();
    println!("4. Ownership - References and Borrowing");
    println!();

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
    // the dereference operator, *. (Chapter 15)

    // We call having references as function parameters 'borrowing'.
    // So what happens if we try to modify something we’re borrowing? It doesn’t work!
    change_borrowed(&hello_str);

    // We can fix the error with just a small tweak:
    let mut hello_mut = String::from("hello");
    change_borrowed_mutable(&mut hello_mut);
    // First, we had to change s to be mut. Then we had to create a mutable reference with &mut s
    // and accept a mutable reference with some_string: &mut String.

    // But mutable references have one big restriction: you can have only one mutable reference to
    // a particular piece of data in a particular scope. This code will fail:
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("r1 = {}", r1);

    // The benefit of having this restriction is that Rust can prevent data races at compile time:
    // * Two or more pointers access the same data at the same time.
    // * At least one of the pointers is being used to write to the data.
    // * There’s no mechanism being used to synchronize access to the data.

    // We can use curly brackets to create a new scope, allowing for multiple mutable references,
    // just not simultaneous ones:
    let mut s2 = String::from("hello");
    {
        let r1 = &mut s2;
        println!("r1 = {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s2;
    println!("r2 = {}", r2);

    // We also cannot have a mutable reference while we have an immutable one:
    let mut s3 = String::from("hello");
    let r3 = &s3; // no problem
    let r4 = &s3; // no problem
    // Compile error[E0502]: cannot borrow `s3` as mutable because it is also borrowed as immutable
    // let r5 = &mut s3; // BIG PROBLEM
    println!("{}, {}", r3, r4);

    // any borrow must last for less than the scope of the owner
}

fn main() {
    println!();
    println!("Learning Rust!");

    ch3_common_concepts_variables();
    ch3_common_concepts_scalar_data_types();
    ch3_common_concepts_compound_data_types();
    ch3_common_concepts_control_flow();

    ch4_ownership_move_with_variables();
    ch4_ownership_move_with_functions();
    ch4_ownership_borrow();
}


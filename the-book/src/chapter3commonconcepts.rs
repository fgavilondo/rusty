use crate::typeutils;

pub(crate) fn variables() {
    // You declare constants using the const keyword instead of the let keyword, and the type of the
    // value MUST be annotated:
    const CONST_FIVE: u32 = 5;

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
    let dashes = "---"; // String literals are slices, not proper strings! (see below)
    println!("dashes as &str: {}", dashes);
    let dashes = dashes.len();
    println!("shadowed dashes:  {}", dashes);

    // Note: String is not the same as &str
    // str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
    // Since the size is unknown, one can only handle it behind a pointer.
    // This means that str most commonly appears as &str: a reference to some UTF-8 data,
    // normally called a "string slice" or just a "slice".
    // As opposed to str, the slice data structure (&str) stores the starting position and the length of the slice.

    // String is the dynamic heap string type, like Vec: use it when you need to own or modify
    // your string data. String keeps the buffer and is very practical to use.

    // &str is lightweight and should be used to "look" into strings.
    // You can search, split, parse, and even replace chunks without needing to allocate new memory.

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("string slices: {} {}", hello, world);

    let mut dashes: String = String::from("---");
    println!("dashes as String: {}", dashes);
    dashes.push('=');
    println!("dashes modified: {}", dashes); // ---=
    let str_slice = &dashes[1..];
    println!("slice 3..len: {}", str_slice);
    dashes.clear();
    println!("dashes cleared: {}", dashes);

    // String slices, as you might imagine, are specific to strings.
    // But there’s a more general slice type, too. Consider this array:

    let a = [1, 2, 3, 4, 5];
    let arr_slice = &a[1..3];
    println!("array slice: {:?}", arr_slice); // [2, 3]
}

pub(crate) fn scalar_data_types() {
    println!();
    println!("3.2.1 Scalar Data Types");
    println!();

    println!("Integers:");
    let i = 1000;
    println!("i = {}", i);
    println!("Type of i = {}", typeutils::type_of(&i));

    // let int_overflow: u8 = 300; // compile error

    println!("Floating Point:");
    let f = 2.0;
    println!("f = {}", f);
    println!("Type of f = {}", typeutils::type_of(&f));

    println!("Booleans:");
    let b = true;
    println!("b = {}", b);
    println!("Type of b = {}", typeutils::type_of(&b));

    println!("Characters:");
    let c = '😻';
    println!("c = {}", c);
    println!("Type of c = {}", typeutils::type_of(&c));
}

fn each_plus_one(arr: &mut [i32; 5]) {
    for i in 0..arr.len() {
        arr[i] = arr[i] + 1;
    }
}

pub(crate) fn compound_data_types_tuples_and_arrays() {
    println!();
    println!("3.2.2 Compound Data Types");
    println!();

    println!("Tuples:");
    // A tuple is a general way of grouping together a number of values with a variety of types
    // into one compound type. Tuples have a fixed length:
    // once declared, they cannot grow or shrink in size.
    let tup1 = (100, 1.4, 'z');
    println!("tup1 = {:?}", tup1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure it:
    let (i, f, c) = tup1;
    println!("i = {}, f = {}, c = {}", i, f, c);
    // In addition to destructuring through pattern matching, we can access a tuple element directly
    // by using a period (.) followed by the index of the value we want to access:
    let tup1_1 = tup1.1;
    println!("tup1_1 = {}", tup1_1);

    let tup2: (i32, f64, u8) = (200, 2.4, 2); // with explicit type annotation
    println!("tup2 (pretty print) = {:#?}", tup2);

    // tuple with only one element:
    let tuple_of_one: (i32, ) = (200, );
    println!("tuple_of_one (pretty print) = {:#?}", tuple_of_one);

    println!();
    println!("Arrays:");
    // Arrays are useful when you want your data allocated on the stack rather than the heap,
    // or when you want to ensure you always have a fixed number of elements.
    // As opposed to Tuples, all values in an Array must have the same type.
    let arr1 = [1, 2, 3, 4, 5];
    println!("arr1 = {:?}", arr1);
    println!("Fist elem: {}", arr1[0]);
    println!("Last elem: {}", arr1[4]);
    // Runtime exception 'index out of bounds: the len is 5 but the index is 5'
    // println!("{}", arr1[5]);

    // Create an array that contains the same value for each element:
    let mut arr2 = [3; 5]; // same as let a = [3, 3, 3, 3, 3];
    println!("[3; 5] = {:?}", arr2);

    // iterating over arrays and modifying elements

    each_plus_one(&mut arr2);
    println!("After each_plus_one() = {:?}", arr2); //  [4, 4, 4, 4, 4];

    println!();
    println!("Vector (Note: not a Compound data type!)");

    // Note: vectors are the heap equivalent of arrays, they can grow or shrink in size:
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    v.push(6);
    println!("v = {:?}", v);
}

pub(crate) fn control_flow() {
    println!();
    println!("3.4 Control Flow");
    println!();

    println!("if expressions:");
    let number = 12;
    if number % 4 == 0 { // good old modulo division trick
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

    // Note there is no Ternary Conditional Operator in Rust! Must use if-expression as above.
    // syntax error:
    // let i = condition? 5 : 6;

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
    println!("looping through an array with 'for'");
    let mut arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("array elem value: {}", element);
    }

    for i in 0..arr.len() {
        println!("array elem {} = {}", i, arr[i]);
        arr[i] = arr[i] + 1;
        println!("  array elem {} = {}", i, arr[i]);
    }

    for number in 1..4 { // range
        println!("{}", number);
    }
    println!("LIFTOFF!!!");

    for number in (1..4).rev() { // reverse range
        println!("{}", number);
    }
    println!("LIFTOFF!!!");

    println!();
    println!("looping through a vector with 'for'");
    let v = vec![1, 2, 3, 4, 5];
    for element in v.iter() {
        println!("vector elem value: {}", element);
    }
}
